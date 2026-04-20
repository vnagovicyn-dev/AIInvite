use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api::health, app::state::AppState, docs::openapi::ApiDoc};

pub fn build_router(state: AppState) -> Router {
    let swagger_router = SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi());

    Router::new()
        .merge(swagger_router)
        .route("/api/health", get(health::health))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
