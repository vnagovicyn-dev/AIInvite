use axum::Json;
use utoipa::OpenApi;

use crate::{
    api::auth::{__path_login, __path_me, __path_register},
    api::events::{
        __path_create_event, __path_delete_event, __path_get_event, __path_list_events,
        __path_publish_event, __path_unpublish_event, __path_update_event,
    },
    api::guests::{
        __path_create_guest, __path_delete_guest, __path_get_guest, __path_import_guests,
        __path_list_guests, __path_update_guest,
    },
    api::health::{__path_health, HealthResponse},
    api::page_sections::{
        __path_create_page_section, __path_delete_page_section, __path_get_page_section,
        __path_list_page_sections, __path_reorder_page_sections, __path_update_page_section,
    },
    api::public::{__path_get_public_event_page, __path_submit_public_rsvp},
    api::rsvp::{
        __path_get_rsvp_form, __path_get_rsvp_response, __path_list_rsvp_responses,
        __path_upsert_rsvp_form,
    },
    api::templates::{__path_get_template, __path_list_categories, __path_list_templates},
    common::error::ErrorResponse,
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
    dto::events::{CreateEventRequest, EventListResponse, EventResponse, UpdateEventRequest},
    dto::guests::{
        CreateGuestRequest, GuestImportResponse, GuestListResponse, GuestResponse,
        UpdateGuestRequest,
    },
    dto::page_sections::{
        CreatePageSectionRequest, PageSectionListResponse, PageSectionResponse,
        ReorderPageSectionsRequest, UpdatePageSectionRequest,
    },
    dto::public::{
        PublicEventPageResponse, PublicGuestContextResponse, PublicPageSectionResponse,
        PublicRsvpFormResponse, PublicRsvpQuestionResponse,
    },
    dto::rsvp::{
        PublicSubmitRsvpRequest, PublicSubmitRsvpResponse, RsvpFormResponse, RsvpQuestionInput,
        RsvpQuestionResponse, RsvpResponseItem, RsvpResponsesAggregates, RsvpResponsesListResponse,
        UpsertRsvpFormRequest,
    },
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
        list_guests,
        create_guest,
        get_guest,
        update_guest,
        delete_guest,
        import_guests,
        create_page_section,
        list_page_sections,
        get_page_section,
        update_page_section,
        delete_page_section,
        reorder_page_sections,
        get_rsvp_form,
        upsert_rsvp_form,
        list_rsvp_responses,
        get_rsvp_response,
        get_public_event_page,
        submit_public_rsvp
    ),
    components(
        schemas(
            AuthResponse,
            CreateEventRequest,
            CreateGuestRequest,
            CreatePageSectionRequest,
            ErrorResponse,
            EventListResponse,
            EventResponse,
            GuestImportResponse,
            GuestListResponse,
            GuestResponse,
            HealthResponse,
            LoginRequest,
            PageSectionListResponse,
            PageSectionResponse,
            ReorderPageSectionsRequest,
            PublicEventPageResponse,
            PublicGuestContextResponse,
            PublicPageSectionResponse,
            PublicRsvpFormResponse,
            PublicRsvpQuestionResponse,
            PublicSubmitRsvpRequest,
            PublicSubmitRsvpResponse,
            RegisterRequest,
            RsvpFormResponse,
            RsvpQuestionInput,
            RsvpQuestionResponse,
            RsvpResponseItem,
            RsvpResponsesAggregates,
            RsvpResponsesListResponse,
            TemplateCategoryResponse,
            TemplateItemResponse,
            TemplateListResponse,
            UpsertRsvpFormRequest,
            UpdateEventRequest,
            UpdateGuestRequest,
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
        (name = "Guests", description = "Authenticated guest management"),
        (name = "Page Sections", description = "Authenticated page section management"),
        (name = "RSVP", description = "RSVP form and response management"),
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
