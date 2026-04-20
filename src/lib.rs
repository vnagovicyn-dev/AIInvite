pub mod api;
pub mod app;
pub mod common;
pub mod db;
pub mod docs;
pub mod domain;
pub mod dto;
pub mod repos;
pub mod services;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_target(false)
        .compact()
        .init();
}
