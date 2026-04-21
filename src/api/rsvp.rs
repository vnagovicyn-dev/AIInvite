use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    app::state::AppState,
    common::{
        auth::CurrentUser,
        error::{AppError, ErrorResponse},
    },
    dto::rsvp::{
        RsvpFormResponse, RsvpResponseItem, RsvpResponsesListQuery, RsvpResponsesListResponse,
        UpsertRsvpFormRequest,
    },
    services::rsvp_service,
};

#[utoipa::path(
    get,
    path = "/api/events/{id}/rsvp-form",
    tag = "RSVP",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    responses(
        (status = 200, description = "RSVP form", body = RsvpFormResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn get_rsvp_form(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
) -> Result<Json<RsvpFormResponse>, AppError> {
    let form = rsvp_service::get_form(&state.pool, current_user.id, event_id).await?;
    Ok(Json(form))
}

#[utoipa::path(
    put,
    path = "/api/events/{id}/rsvp-form",
    tag = "RSVP",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    request_body = UpsertRsvpFormRequest,
    responses(
        (status = 200, description = "RSVP form updated", body = RsvpFormResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 409, description = "Question code conflict", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn upsert_rsvp_form(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
    Json(payload): Json<UpsertRsvpFormRequest>,
) -> Result<Json<RsvpFormResponse>, AppError> {
    let form = rsvp_service::upsert_form(&state.pool, current_user.id, event_id, payload).await?;
    Ok(Json(form))
}

#[utoipa::path(
    get,
    path = "/api/events/{id}/rsvp-responses",
    tag = "RSVP",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id"),
        RsvpResponsesListQuery
    ),
    responses(
        (status = 200, description = "RSVP responses", body = RsvpResponsesListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn list_rsvp_responses(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
    Query(query): Query<RsvpResponsesListQuery>,
) -> Result<Json<RsvpResponsesListResponse>, AppError> {
    let response =
        rsvp_service::list_responses(&state.pool, current_user.id, event_id, query).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/events/{id}/rsvp-responses/{response_id}",
    tag = "RSVP",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id"),
        ("response_id" = Uuid, Path, description = "RSVP response id")
    ),
    responses(
        (status = 200, description = "RSVP response details", body = RsvpResponseItem),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "RSVP response not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn get_rsvp_response(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((event_id, response_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<RsvpResponseItem>, AppError> {
    let response = rsvp_service::get_response(&state.pool, current_user.id, event_id, response_id)
        .await?
        .ok_or_else(|| AppError::not_found("rsvp response not found"))?;

    Ok(Json(response))
}
