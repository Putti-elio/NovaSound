use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use crate::migrations::{apply_migrations, reset_database};

pub async fn create_test_pool() -> Result<Pool, String> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .map_err(|_| "TEST_DATABASE_URL or DATABASE_URL not set".to_string())?;

    let mut cfg = Config::new();
    cfg.url = Some(database_url);

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| format!("Failed to create test pool: {e}"))?;

    let mut client = pool
        .get()
        .await
        .map_err(|e| format!("Failed to get test client: {e}"))?;

    reset_database(&client)
        .await
        .map_err(|e| format!("Failed to reset test schema: {e}"))?;

    apply_migrations(&mut client)
        .await
        .map_err(|e| format!("Failed to apply test migrations: {e}"))?;

    Ok(pool)
}

#[macro_export]
macro_rules! get_test_pool {
    () => {
        match $crate::tests::test_helpers::create_test_pool().await {
            | Ok(pool) => pool,
            | Err(e) => {
                eprintln!("Skipping test — no test database available: {e}");
                return;
            },
        }
    };
}
