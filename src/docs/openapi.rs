use axum::Json;
use utoipa::OpenApi;

use crate::{
    api::auth::{__path_login, __path_me, __path_register},
    api::events::{
        __path_create_event, __path_delete_event, __path_get_event, __path_list_events,
        __path_publish_event, __path_unpublish_event, __path_update_event,
    },
    api::health::{__path_health, HealthResponse},
    api::page_sections::{
        __path_create_page_section, __path_delete_page_section, __path_get_page_section,
        __path_list_page_sections, __path_update_page_section,
    },
    api::public::__path_get_public_event_page,
    api::templates::{__path_get_template, __path_list_categories, __path_list_templates},
    common::error::ErrorResponse,
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
    dto::events::{CreateEventRequest, EventListResponse, EventResponse, UpdateEventRequest},
    dto::page_sections::{
        CreatePageSectionRequest, PageSectionListResponse, PageSectionResponse,
        UpdatePageSectionRequest,
    },
    dto::public::{PublicEventPageResponse, PublicPageSectionResponse},
    dto::templates::{TemplateCategoryResponse, TemplateItemResponse, TemplateListResponse},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
        register,
        login,
        me,
        list_templates,
        get_template,
        list_categories,
        create_event,
        list_events,
        get_event,
        update_event,
        delete_event,
        publish_event,
        unpublish_event,
        create_page_section,
        list_page_sections,
        get_page_section,
        update_page_section,
        delete_page_section,
        get_public_event_page
    ),
    components(
        schemas(
            AuthResponse,
            CreateEventRequest,
            CreatePageSectionRequest,
            ErrorResponse,
            EventListResponse,
            EventResponse,
            HealthResponse,
            LoginRequest,
            PageSectionListResponse,
            PageSectionResponse,
            PublicEventPageResponse,
            PublicPageSectionResponse,
            RegisterRequest,
            TemplateCategoryResponse,
            TemplateItemResponse,
            TemplateListResponse,
            UpdateEventRequest,
            UpdatePageSectionRequest,
            UserResponse
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Service health endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Templates", description = "Public templates catalog"),
        (name = "Events", description = "Authenticated event management"),
        (name = "Page Sections", description = "Authenticated page section management"),
        (name = "Public", description = "Public invitation page endpoints")
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
