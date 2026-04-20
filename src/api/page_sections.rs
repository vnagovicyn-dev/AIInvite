use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    app::state::AppState,
    common::{
        auth::CurrentUser,
        error::{AppError, ErrorResponse},
    },
    dto::page_sections::{
        CreatePageSectionRequest, PageSectionListResponse, PageSectionResponse,
        UpdatePageSectionRequest,
    },
    services::page_sections_service,
};

#[utoipa::path(
    post,
    path = "/api/events/{event_id}/sections",
    tag = "Page Sections",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("event_id" = Uuid, Path, description = "Event id")
    ),
    request_body = CreatePageSectionRequest,
    responses(
        (status = 201, description = "Page section created successfully", body = PageSectionResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 409, description = "Position already exists", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn create_page_section(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
    Json(payload): Json<CreatePageSectionRequest>,
) -> Result<(StatusCode, Json<PageSectionResponse>), AppError> {
    let section =
        page_sections_service::create(&state.pool, current_user.id, event_id, payload).await?;
    Ok((StatusCode::CREATED, Json(section)))
}

#[utoipa::path(
    get,
    path = "/api/events/{event_id}/sections",
    tag = "Page Sections",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("event_id" = Uuid, Path, description = "Event id")
    ),
    responses(
        (status = 200, description = "Event page sections", body = PageSectionListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn list_page_sections(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
) -> Result<Json<PageSectionListResponse>, AppError> {
    let sections = page_sections_service::list(&state.pool, current_user.id, event_id).await?;
    Ok(Json(sections))
}

#[utoipa::path(
    get,
    path = "/api/events/{event_id}/sections/{id}",
    tag = "Page Sections",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("event_id" = Uuid, Path, description = "Event id"),
        ("id" = Uuid, Path, description = "Section id")
    ),
    responses(
        (status = 200, description = "Page section details", body = PageSectionResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Page section not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn get_page_section(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((event_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<PageSectionResponse>, AppError> {
    let section = page_sections_service::get_by_id(&state.pool, current_user.id, event_id, id)
        .await?
        .ok_or_else(|| AppError::not_found("page section not found"))?;

    Ok(Json(section))
}

#[utoipa::path(
    patch,
    path = "/api/events/{event_id}/sections/{id}",
    tag = "Page Sections",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("event_id" = Uuid, Path, description = "Event id"),
        ("id" = Uuid, Path, description = "Section id")
    ),
    request_body = UpdatePageSectionRequest,
    responses(
        (status = 200, description = "Page section updated successfully", body = PageSectionResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Page section not found", body = ErrorResponse),
        (status = 409, description = "Position already exists", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn update_page_section(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((event_id, id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdatePageSectionRequest>,
) -> Result<Json<PageSectionResponse>, AppError> {
    let section =
        page_sections_service::update(&state.pool, current_user.id, event_id, id, payload)
            .await?
            .ok_or_else(|| AppError::not_found("page section not found"))?;

    Ok(Json(section))
}

#[utoipa::path(
    delete,
    path = "/api/events/{event_id}/sections/{id}",
    tag = "Page Sections",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("event_id" = Uuid, Path, description = "Event id"),
        ("id" = Uuid, Path, description = "Section id")
    ),
    responses(
        (status = 204, description = "Page section deleted successfully"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Page section not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn delete_page_section(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((event_id, id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let deleted = page_sections_service::delete(&state.pool, current_user.id, event_id, id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found("page section not found"))
    }
}
