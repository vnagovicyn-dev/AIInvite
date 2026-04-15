use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::models::ErrorResponse;

pub struct AppError(pub StatusCode, pub String);

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self(StatusCode::BAD_REQUEST, message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self(StatusCode::NOT_FOUND, message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, Json(ErrorResponse { error: self.1 })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(error: sqlx::migrate::MigrateError) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}
