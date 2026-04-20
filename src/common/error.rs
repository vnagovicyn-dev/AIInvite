use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("internal server error")]
    Internal,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
}

impl AppError {
    pub fn internal() -> Self {
        Self::Internal
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn public_message(&self) -> String {
        match self {
            Self::Internal => "internal server error".to_string(),
            Self::BadRequest(message) | Self::NotFound(message) => message.clone(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = ErrorResponse {
            error: self.public_message(),
        };

        (status_code, Json(body)).into_response()
    }
}
