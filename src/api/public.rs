use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    app::state::AppState,
    common::error::{AppError, ErrorResponse},
    dto::{
        public::PublicEventPageResponse,
        rsvp::{PublicSubmitRsvpRequest, PublicSubmitRsvpResponse},
    },
    services::{page_service, rsvp_service},
};

#[derive(Debug, Deserialize)]
pub struct PublicPageQuery {
    pub invite_token: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/public/{slug}",
    tag = "Public",
    params(
        ("slug" = String, Path, description = "Published event slug"),
        ("invite_token" = Option<String>, Query, description = "Guest invite token for personalization")
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
    Query(query): Query<PublicPageQuery>,
) -> Result<Json<PublicEventPageResponse>, AppError> {
    let page =
        page_service::get_public_page_by_slug(&state.pool, &slug, query.invite_token.as_deref())
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
