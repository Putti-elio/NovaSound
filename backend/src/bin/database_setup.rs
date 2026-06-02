use anyhow::Context;
use deadpool_postgres::{Config, Runtime};
use rust::migrations::{apply_migrations, reset_database};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let reset = std::env::args().any(|arg| arg == "--reset");
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL env var not set")?;

    let mut cfg = Config::new();
    cfg.url = Some(database_url);

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("Failed to create connection pool")?;

    let mut client = pool
        .get()
        .await
        .context("Failed to get initial connection")?;

    if reset {
        reset_database(&client)
            .await
            .context("Failed to reset database")?;
    }

    apply_migrations(&mut client)
        .await
        .context("Failed to apply migrations")?;

    Ok(())
}
