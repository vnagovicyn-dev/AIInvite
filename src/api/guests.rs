use axum::{
    extract::{Multipart, Path, Query, State},
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
    dto::guests::{
        CreateGuestRequest, GuestImportResponse, GuestListQuery, GuestListResponse, GuestResponse,
        UpdateGuestRequest,
    },
    services::guest_service,
};

#[utoipa::path(
    get,
    path = "/api/events/{id}/guests",
    tag = "Guests",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id"),
        GuestListQuery
    ),
    responses(
        (status = 200, description = "Event guests", body = GuestListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn list_guests(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
    Query(query): Query<GuestListQuery>,
) -> Result<Json<GuestListResponse>, AppError> {
    let response = guest_service::list(&state.pool, current_user.id, event_id, query).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/events/{id}/guests",
    tag = "Guests",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    request_body = CreateGuestRequest,
    responses(
        (status = 201, description = "Guest created successfully", body = GuestResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn create_guest(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
    Json(payload): Json<CreateGuestRequest>,
) -> Result<(StatusCode, Json<GuestResponse>), AppError> {
    let guest = guest_service::create(&state.pool, current_user.id, event_id, payload).await?;
    Ok((StatusCode::CREATED, Json(guest)))
}

#[utoipa::path(
    get,
    path = "/api/events/{id}/guests/{guest_id}",
    tag = "Guests",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id"),
        ("guest_id" = Uuid, Path, description = "Guest id")
    ),
    responses(
        (status = 200, description = "Guest details", body = GuestResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Guest not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn get_guest(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((event_id, guest_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<GuestResponse>, AppError> {
    let guest = guest_service::get_by_id(&state.pool, current_user.id, event_id, guest_id)
        .await?
        .ok_or_else(|| AppError::not_found("guest not found"))?;

    Ok(Json(guest))
}

#[utoipa::path(
    patch,
    path = "/api/events/{id}/guests/{guest_id}",
    tag = "Guests",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id"),
        ("guest_id" = Uuid, Path, description = "Guest id")
    ),
    request_body = UpdateGuestRequest,
    responses(
        (status = 200, description = "Guest updated successfully", body = GuestResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Guest not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn update_guest(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((event_id, guest_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateGuestRequest>,
) -> Result<Json<GuestResponse>, AppError> {
    let guest = guest_service::update(&state.pool, current_user.id, event_id, guest_id, payload)
        .await?
        .ok_or_else(|| AppError::not_found("guest not found"))?;

    Ok(Json(guest))
}

#[utoipa::path(
    delete,
    path = "/api/events/{id}/guests/{guest_id}",
    tag = "Guests",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id"),
        ("guest_id" = Uuid, Path, description = "Guest id")
    ),
    responses(
        (status = 204, description = "Guest deleted successfully"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Guest not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn delete_guest(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((event_id, guest_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let deleted = guest_service::delete(&state.pool, current_user.id, event_id, guest_id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found("guest not found"))
    }
}

#[utoipa::path(
    post,
    path = "/api/events/{id}/guests/import",
    tag = "Guests",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Event id")
    ),
    request_body(
        content = String,
        content_type = "multipart/form-data",
        description = "CSV file upload in multipart field `file`"
    ),
    responses(
        (status = 200, description = "Guests imported", body = GuestImportResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn import_guests(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(event_id): Path<Uuid>,
    multipart: Multipart,
) -> Result<Json<GuestImportResponse>, AppError> {
    let response =
        guest_service::import_csv(&state.pool, current_user.id, event_id, multipart).await?;
    Ok(Json(response))
}
