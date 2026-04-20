use axum::Json;
use utoipa::OpenApi;

use crate::{
    api::health::{health, HealthResponse},
    common::error::ErrorResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(health),
    components(schemas(HealthResponse, ErrorResponse)),
    tags(
        (name = "Health", description = "Service health endpoints")
    )
)]
pub struct ApiDoc;

pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
