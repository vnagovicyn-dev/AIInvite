use std::net::SocketAddr;

use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::{config::Config, handlers, models::AppState};

pub fn build_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(handlers::api_root))
        .route("/api", get(handlers::api_root))
        .route("/health", get(handlers::health))
        .route(
            "/api/healthchecks",
            get(handlers::list_healthchecks).post(handlers::create_healthcheck),
        )
        .route(
            "/api/healthchecks/{id}",
            get(handlers::get_healthcheck)
                .put(handlers::update_healthcheck)
                .delete(handlers::delete_healthcheck),
        )
        .with_state(AppState { pool })
}

pub async fn serve(config: Config, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, build_router(pool))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}
