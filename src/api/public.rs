use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    app::state::AppState,
    common::error::{AppError, ErrorResponse},
    dto::{
        public::PublicEventPageResponse,
        rsvp::{PublicSubmitRsvpRequest, PublicSubmitRsvpResponse},
    },
    services::{page_service, rsvp_service},
};

#[utoipa::path(
    get,
    path = "/api/public/{slug}",
    tag = "Public",
    params(
        ("slug" = String, Path, description = "Published event slug")
    ),
    responses(
        (status = 200, description = "Published event page", body = PublicEventPageResponse),
        (status = 404, description = "Published event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn get_public_event_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicEventPageResponse>, AppError> {
    let page = page_service::get_public_page_by_slug(&state.pool, &slug)
        .await?
        .ok_or_else(|| AppError::not_found("event not found"))?;

    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/api/public/{slug}/rsvp",
    tag = "Public",
    params(
        ("slug" = String, Path, description = "Published event slug")
    ),
    request_body = PublicSubmitRsvpRequest,
    responses(
        (status = 201, description = "RSVP submitted", body = PublicSubmitRsvpResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Published event not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn submit_public_rsvp(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(payload): Json<PublicSubmitRsvpRequest>,
) -> Result<(StatusCode, Json<PublicSubmitRsvpResponse>), AppError> {
    let response = rsvp_service::submit_public_response(&state.pool, &slug, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
