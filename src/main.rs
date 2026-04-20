use aiinvite::{
    app::{config::Config, router::build_router, state::AppState},
    db::pool,
    init_tracing,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = Config::from_env()?;
    let address = config.address();
    let pool = pool::create_pool(&config.database_url).await?;
    pool::run_migrations(&pool).await?;

    let state = AppState::new(config.app_name.clone(), pool);
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
