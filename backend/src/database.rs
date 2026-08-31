use deadpool_postgres::{Config, Pool, Runtime};
use function_name::named;
use log::{error, info};
use tokio_postgres::NoTls;

use crate::errors::AppResult;

#[named]
pub async fn init_database(database_url: &str) -> AppResult<Pool> {
    let mut cfg = Config::new();
    cfg.url = Some(database_url.to_string());

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|err| {
            error!(
                "Failed to create connection pool: {}. At {}::{}",
                err,
                file!(),
                function_name!()
            );
            crate::errors::AppError::Internal(crate::utils::log_and_context_error(
                err,
                "Failed to create connection pool",
                file!(),
                function_name!(),
            ))
        })?;

    let _connection = pool.get().await.map_err(|err| {
        error!(
            "Database couldn't be initialized: {}. At {}::{}",
            err,
            file!(),
            function_name!()
        );
        crate::errors::AppError::Internal(crate::utils::log_and_context_error(
            err,
            "Failed to get initial connection",
            file!(),
            function_name!(),
        ))
    })?;

    info!("Database connection pool initialized successfully!");
    Ok(pool)
}
