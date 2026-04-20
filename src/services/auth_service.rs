use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::{
        auth::{create_access_token, JwtSettings},
        error::AppError,
    },
    domain::auth::{NewUser, User},
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
    repos::users_repo,
};

pub async fn register(pool: &PgPool, payload: RegisterRequest) -> Result<User, AppError> {
    validate_email(&payload.email)?;
    validate_password(&payload.password)?;

    if users_repo::find_by_email(pool, &payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::conflict("email already exists"));
    }

    let password_hash = hash_password(&payload.password)?;
    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: payload.email.trim().to_lowercase(),
        password_hash,
        full_name: normalize_full_name(payload.full_name),
        role: "user".to_string(),
    };

    let user = users_repo::create(pool, &new_user)
        .await
        .map_err(map_user_write_error)?;

    Ok(user.into_public_user())
}

pub async fn login(
    pool: &PgPool,
    jwt_settings: &JwtSettings,
    payload: LoginRequest,
) -> Result<AuthResponse, AppError> {
    validate_email(&payload.email)?;
    validate_login_password(&payload.password)?;

    let user = users_repo::find_by_email(pool, &payload.email)
        .await?
        .ok_or_else(|| AppError::unauthorized("invalid credentials"))?;

    verify_password(&payload.password, &user.password_hash)?;

    let public_user = user.into_public_user();
    let access_token = create_access_token(&public_user, jwt_settings)?;

    Ok(AuthResponse {
        access_token,
        token_type: "Bearer".to_string(),
        user: UserResponse::from(public_user),
    })
}

fn validate_email(email: &str) -> Result<(), AppError> {
    let normalized_email = email.trim();
    if normalized_email.is_empty() || !normalized_email.contains('@') {
        return Err(AppError::bad_request("email is invalid"));
    }

    Ok(())
}

fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::bad_request(
            "password must be at least 8 characters long",
        ));
    }

    Ok(())
}

fn validate_login_password(password: &str) -> Result<(), AppError> {
    if password.is_empty() {
        return Err(AppError::bad_request("password must not be empty"));
    }

    Ok(())
}

fn normalize_full_name(full_name: Option<String>) -> Option<String> {
    full_name.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|password_hash| password_hash.to_string())
        .map_err(|error| {
            tracing::error!(error = %error, "failed to hash password");
            AppError::internal()
        })
}

fn verify_password(password: &str, password_hash: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|error| {
        tracing::error!(error = %error, "stored password hash is invalid");
        AppError::internal()
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::unauthorized("invalid credentials"))
}

fn map_user_write_error(error: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(database_error) = &error {
        if database_error.code().as_deref() == Some("23505") {
            return AppError::conflict("email already exists");
        }
    }

    error.into()
}
