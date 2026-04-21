use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{auth, events, health, page_sections, public, templates},
    app::state::AppState,
    docs::openapi::ApiDoc,
};

pub fn build_router(state: AppState) -> Router {
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
            "/api/events/:event_id/sections",
            post(page_sections::create_page_section).get(page_sections::list_page_sections),
        )
        .route(
            "/api/events/:event_id/sections/:id",
            get(page_sections::get_page_section)
                .patch(page_sections::update_page_section)
                .delete(page_sections::delete_page_section),
        )
        .route("/api/templates", get(templates::list_templates))
        .route("/api/templates/categories", get(templates::list_categories))
        .route("/api/templates/:id", get(templates::get_template))
        .route("/api/public/:slug", get(public::get_public_event_page))
        .route("/api/health", get(health::health))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
