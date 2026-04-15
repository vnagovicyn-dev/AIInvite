pub mod app;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod site;

use std::env;

pub enum Command {
    Serve,
    Migrate,
}

pub fn parse_command() -> Result<Command, String> {
    match env::args().nth(1).as_deref() {
        None | Some("serve") => Ok(Command::Serve),
        Some("migrate") => Ok(Command::Migrate),
        Some(other) => Err(format!(
            "unknown command: {other}. Use `serve` or `migrate`."
        )),
    }
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aiinvite=info,tower_http=info".into()),
        )
        .with_target(false)
        .compact()
        .init();
}
