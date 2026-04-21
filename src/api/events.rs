use axum::{
    extract::{Path, Query, State},
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
    dto::events::{
        CreateEventRequest, EventListQuery, EventListResponse, EventResponse, UpdateEventRequest,
    },
    services::event_service,
};

#[utoipa::path(
    post,
    path = "/api/events",
    tag = "Events",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateEventRequest,
    responses(
        (status = 201, description = "Event created successfully", body = EventResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 409, description = "Slug already exists", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn create_event(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<EventResponse>), AppError> {
    let event = event_service::create(&state.pool, current_user.id, payload).await?;
    Ok((StatusCode::CREATED, Json(event)))
}

#[utoipa::path(
    get,
    path = "/api/events",
    tag = "Events",
    security(
        ("bearer_auth" = [])
    ),
    params(EventListQuery),
    responses(
        (status = 200, description = "Current user's events", body = EventListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn list_events(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<EventListQuery>,
) -> Result<Json<EventListResponse>, AppError> {
    let response = event_service::list(&state.pool, current_user.id, query).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/events/{id}",
    tag = "Events",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    responses(
        (status = 200, description = "Event details", body = EventResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn get_event(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<EventResponse>, AppError> {
    let event = event_service::get_by_id(&state.pool, current_user.id, id)
        .await?
        .ok_or_else(|| AppError::not_found("event not found"))?;

    Ok(Json(event))
}

#[utoipa::path(
    patch,
    path = "/api/events/{id}",
    tag = "Events",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    request_body = UpdateEventRequest,
    responses(
        (status = 200, description = "Event updated successfully", body = EventResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 409, description = "Slug already exists", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn update_event(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateEventRequest>,
) -> Result<Json<EventResponse>, AppError> {
    let event = event_service::update(&state.pool, current_user.id, id, payload)
        .await?
        .ok_or_else(|| AppError::not_found("event not found"))?;

    Ok(Json(event))
}

#[utoipa::path(
    delete,
    path = "/api/events/{id}",
    tag = "Events",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    responses(
        (status = 204, description = "Event deleted successfully"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn delete_event(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let deleted = event_service::delete(&state.pool, current_user.id, id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found("event not found"))
    }
}

#[utoipa::path(
    post,
    path = "/api/events/{id}/publish",
    tag = "Events",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    responses(
        (status = 200, description = "Event published successfully", body = EventResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn publish_event(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<EventResponse>, AppError> {
    let event = event_service::publish(&state.pool, current_user.id, id)
        .await?
        .ok_or_else(|| AppError::not_found("event not found"))?;

    Ok(Json(event))
}

#[utoipa::path(
    post,
    path = "/api/events/{id}/unpublish",
    tag = "Events",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    responses(
        (status = 200, description = "Event unpublished successfully", body = EventResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn unpublish_event(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<EventResponse>, AppError> {
    let event = event_service::unpublish(&state.pool, current_user.id, id)
        .await?
        .ok_or_else(|| AppError::not_found("event not found"))?;

    Ok(Json(event))
}
