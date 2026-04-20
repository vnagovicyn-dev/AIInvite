use axum::Json;
use utoipa::OpenApi;

use crate::{
    api::auth::{__path_login, __path_me, __path_register},
    api::health::{__path_health, HealthResponse},
    api::templates::{__path_get_template, __path_list_categories, __path_list_templates},
    common::error::ErrorResponse,
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
    dto::templates::{TemplateCategoryResponse, TemplateItemResponse, TemplateListResponse},
};

#[derive(OpenApi)]
#[openapi(
    paths(health, register, login, me, list_templates, get_template, list_categories),
    components(
        schemas(
            AuthResponse,
            ErrorResponse,
            HealthResponse,
            LoginRequest,
            RegisterRequest,
            TemplateCategoryResponse,
            TemplateItemResponse,
            TemplateListResponse,
            UserResponse
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Service health endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Templates", description = "Public templates catalog")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            utoipa::openapi::security::SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}

pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
