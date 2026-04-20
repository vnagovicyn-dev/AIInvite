use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{auth, health},
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
        .route("/api/health", get(health::health))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
