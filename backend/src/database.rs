use deadpool_postgres::{Config, Pool, Runtime};
use function_name::named;
use log::info;
use tokio_postgres::NoTls;

use crate::create_error;
use crate::errors::AppResult;

#[named]
pub async fn init_database(database_url: &str) -> AppResult<Pool> {
    let mut cfg = Config::new();
    cfg.url = Some(database_url.to_string());

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|err| create_error!(err, "Failed to create connection pool"))?;

    let _connection = pool
        .get()
        .await
        .map_err(|err| create_error!(err, "Failed to get initial connection"))?;

    info!("Database connection pool initialized successfully!");
    Ok(pool)
}
