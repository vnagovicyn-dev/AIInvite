use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app::state::AppState,
    common::error::{AppError, ErrorResponse},
    dto::public::PublicEventPageResponse,
    services::page_service,
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
