use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::state::AppState, common::error::AppError, domain::auth::User};

#[derive(Clone)]
pub struct JwtSettings {
    pub secret: String,
    pub expires_in_minutes: u64,
}

impl JwtSettings {
    pub fn new(secret: String, expires_in_minutes: u64) -> Self {
        Self {
            secret,
            expires_in_minutes,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub full_name: Option<String>,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub role: String,
}

impl CurrentUser {
    pub fn into_user(self) -> User {
        User {
            id: self.id,
            email: self.email,
            full_name: self.full_name,
            role: self.role,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let bearer_token = extract_bearer_token(parts)?;
        let token_data = decode::<Claims>(
            bearer_token,
            &DecodingKey::from_secret(state.jwt.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AppError::unauthorized("unauthorized"))?;

        let claims = token_data.claims;
        let id =
            Uuid::parse_str(&claims.sub).map_err(|_| AppError::unauthorized("unauthorized"))?;

        Ok(Self {
            id,
            email: claims.email,
            full_name: claims.full_name,
            role: claims.role,
        })
    }
}

pub fn create_access_token(user: &User, jwt_settings: &JwtSettings) -> Result<String, AppError> {
    let issued_at = unix_timestamp_now()?;
    let expiration = issued_at + jwt_settings.expires_in_minutes.saturating_mul(60);
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        full_name: user.full_name.clone(),
        role: user.role.clone(),
        exp: expiration as usize,
        iat: issued_at as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt_settings.secret.as_bytes()),
    )
    .map_err(|error| {
        tracing::error!(error = %error, "failed to encode jwt");
        AppError::internal()
    })
}

fn unix_timestamp_now() -> Result<u64, AppError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| {
            tracing::error!(error = %error, "system clock before unix epoch");
            AppError::internal()
        })
}

fn extract_bearer_token(parts: &Parts) -> Result<&str, AppError> {
    let header_value = parts
        .headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("unauthorized"))?;
    let header_str = header_value
        .to_str()
        .map_err(|_| AppError::unauthorized("unauthorized"))?;

    header_str
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::unauthorized("unauthorized"))
}
