use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    app::state::AppState,
    common::error::{AppError, ErrorResponse},
    dto::templates::{
        TemplateCategoryResponse, TemplateItemResponse, TemplateListQuery, TemplateListResponse,
    },
    services::templates_service,
};

#[utoipa::path(
    get,
    path = "/api/templates",
    tag = "Templates",
    params(TemplateListQuery),
    responses(
        (status = 200, description = "Templates catalog page", body = TemplateListResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn list_templates(
    State(state): State<AppState>,
    Query(query): Query<TemplateListQuery>,
) -> Result<Json<TemplateListResponse>, AppError> {
    let response = templates_service::list(&state.pool, query).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/templates/{id}",
    tag = "Templates",
    params(
        ("id" = Uuid, Path, description = "Template id")
    ),
    responses(
        (status = 200, description = "Template details", body = TemplateItemResponse),
        (status = 404, description = "Template not found", body = ErrorResponse),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TemplateItemResponse>, AppError> {
    let template = templates_service::get_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::not_found("template not found"))?;

    Ok(Json(template))
}

#[utoipa::path(
    get,
    path = "/api/templates/categories",
    tag = "Templates",
    responses(
        (status = 200, description = "Unique template categories", body = [TemplateCategoryResponse]),
        (status = 500, description = "Unexpected server error", body = ErrorResponse)
    )
)]
pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<TemplateCategoryResponse>>, AppError> {
    let categories = templates_service::list_categories(&state.pool).await?;
    Ok(Json(categories))
}
