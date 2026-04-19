use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db,
    error::AppError,
    models::{AppState, CreateHealthcheck, DeleteResponse, HealthResponse, RootResponse, UpdateHealthcheck},
};

pub async fn api_root() -> Json<RootResponse> {
    Json(RootResponse {
        service: "rust-postgres-api",
        routes: vec!["/", "/api", "/health", "/api/healthchecks", "/api/healthchecks/:id"],
    })
}

pub async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    let version = db::fetch_database_version(&state.pool).await?;

    Ok(Json(HealthResponse {
        status: "ok",
        database: "connected",
        version,
    }))
}

pub async fn list_healthchecks(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::Healthcheck>>, AppError> {
    let items = db::list_healthchecks(&state.pool).await?;
    Ok(Json(items))
}

pub async fn get_healthcheck(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<crate::models::Healthcheck>, AppError> {
    let item = db::get_healthcheck(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::not_found(format!("healthcheck {id} not found")))?;

    Ok(Json(item))
}

pub async fn create_healthcheck(
    State(state): State<AppState>,
    Json(payload): Json<CreateHealthcheck>,
) -> Result<(StatusCode, Json<crate::models::Healthcheck>), AppError> {
    validate_service_name(&payload.service_name)?;
    let record = db::create_healthcheck(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(record)))
}

pub async fn update_healthcheck(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateHealthcheck>,
) -> Result<Json<crate::models::Healthcheck>, AppError> {
    validate_service_name(&payload.service_name)?;
    let record = db::update_healthcheck(&state.pool, id, &payload)
        .await?
        .ok_or_else(|| AppError::not_found(format!("healthcheck {id} not found")))?;

    Ok(Json(record))
}

pub async fn delete_healthcheck(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<DeleteResponse>, AppError> {
    let deleted = db::delete_healthcheck(&state.pool, id).await?;
    if !deleted {
        return Err(AppError::not_found(format!("healthcheck {id} not found")));
    }

    Ok(Json(DeleteResponse { deleted: true }))
}

fn validate_service_name(service_name: &str) -> Result<(), AppError> {
    if service_name.trim().is_empty() {
        return Err(AppError::bad_request("service_name must not be empty"));
    }

    Ok(())
}
