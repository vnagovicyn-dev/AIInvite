use axum::{extract::State, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    app::state::AppState,
    common::error::{AppError, ErrorResponse},
};

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: String,
}

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: "ok",
        service: state.service_name,
    }))
}
