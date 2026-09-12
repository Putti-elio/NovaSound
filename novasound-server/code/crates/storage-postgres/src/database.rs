use deadpool_postgres::{Config, Pool, Runtime};
use log::info;
use tokio_postgres::NoTls;

pub async fn init_database(database_url: &str) -> anyhow::Result<Pool> {
    let mut cfg = Config::new();
    cfg.url = Some(database_url.to_string());

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|err| anyhow::Error::msg(format!("Failed to create connection pool: {err}")))?;

    let _connection = pool
        .get()
        .await
        .map_err(|err| anyhow::Error::msg(format!("Failed to get initial connection: {err}")))?;

    info!("Database connection pool initialized successfully!");
    Ok(pool)
}
