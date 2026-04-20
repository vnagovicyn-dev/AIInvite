use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::health,
    app::state::AppState,
    docs::openapi::{openapi_json, ApiDoc},
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health::health))
        .route("/api/openapi.json", get(openapi_json))
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
