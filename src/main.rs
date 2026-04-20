use aiinvite::{
    app::{config::Config, router::build_router, state::AppState},
    init_tracing,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = Config::from_env();
    let address = config.address();
    let state = AppState::new(config.app_name.clone());
    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(address.as_str()).await?;
    tracing::info!(address = %address, "starting http server");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}
