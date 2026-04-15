use aiinvite::{app::serve, config::Config, db, init_tracing, parse_command, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = Config::from_env();
    let command = parse_command().map_err(std::io::Error::other)?;
    let pool = db::connect_pool(&config.database_url).await?;

    db::run_migrations(&pool).await?;

    match command {
        Command::Migrate => {
            tracing::info!("migrations applied successfully");
            Ok(())
        }
        Command::Serve => serve(config, pool).await,
    }
}
