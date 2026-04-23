use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{auth, events, guests, health, page_sections, public, rsvp, templates},
    app::state::AppState,
    docs::openapi::ApiDoc,
};

pub fn build_router(state: AppState) -> Router {
    let cors_layer = build_cors_layer(&state);
    let swagger_router = SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi());

    Router::new()
        .merge(swagger_router)
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/me", get(auth::me))
        .route(
            "/api/events",
            post(events::create_event).get(events::list_events),
        )
        .route(
            "/api/events/:id",
            get(events::get_event)
                .patch(events::update_event)
                .delete(events::delete_event),
        )
        .route("/api/events/:id/publish", post(events::publish_event))
        .route("/api/events/:id/unpublish", post(events::unpublish_event))
        .route(
            "/api/events/:id/rsvp-form",
            get(rsvp::get_rsvp_form).put(rsvp::upsert_rsvp_form),
        )
        .route(
            "/api/events/:id/rsvp-responses",
            get(rsvp::list_rsvp_responses),
        )
        .route(
            "/api/events/:id/rsvp-responses/:response_id",
            get(rsvp::get_rsvp_response),
        )
        .route(
            "/api/events/:event_id/sections",
            post(page_sections::create_page_section).get(page_sections::list_page_sections),
        )
        .route(
            "/api/events/:event_id/sections/reorder",
            post(page_sections::reorder_page_sections),
        )
        .route(
            "/api/events/:event_id/sections/:id",
            get(page_sections::get_page_section)
                .patch(page_sections::update_page_section)
                .delete(page_sections::delete_page_section),
        )
        .route(
            "/api/events/:id/guests",
            get(guests::list_guests).post(guests::create_guest),
        )
        .route("/api/events/:id/guests/import", post(guests::import_guests))
        .route(
            "/api/events/:id/guests/:guest_id",
            get(guests::get_guest)
                .patch(guests::update_guest)
                .delete(guests::delete_guest),
        )
        .route("/api/templates", get(templates::list_templates))
        .route("/api/templates/categories", get(templates::list_categories))
        .route("/api/templates/:id", get(templates::get_template))
        .route("/api/public/:slug", get(public::get_public_event_page))
        .route("/api/public/:slug/rsvp", post(public::submit_public_rsvp))
        .route("/api/health", get(health::health))
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn build_cors_layer(state: &AppState) -> CorsLayer {
    let allowed_origins = state
        .frontend_origins
        .iter()
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect::<Vec<HeaderValue>>();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
}
