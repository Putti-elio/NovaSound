use deadpool_postgres::{Config, Pool, Runtime};
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;

use crate::migrations::apply_migrations;

static TEST_POOL: OnceCell<Pool> = OnceCell::const_new();
static TEST_SCHEMA_READY: OnceCell<()> = OnceCell::const_new();

pub enum TestSetupError {
    MissingDatabaseUrl,
    SetupFailed(String),
}

pub async fn create_test_pool() -> Result<Pool, TestSetupError> {
    let pool = TEST_POOL
        .get_or_try_init(create_shared_test_pool)
        .await?
        .clone();

    TEST_SCHEMA_READY
        .get_or_try_init(|| init_test_schema(&pool))
        .await?;

    clear_test_data(&pool).await?;

    Ok(pool)
}

async fn create_shared_test_pool() -> Result<Pool, TestSetupError> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .map_err(|_| TestSetupError::MissingDatabaseUrl)?;

    let mut cfg = Config::new();
    cfg.url = Some(database_url);

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| TestSetupError::SetupFailed(format!("Failed to create test pool: {e}")))
}

async fn init_test_schema(pool: &Pool) -> Result<(), TestSetupError> {
    let mut client = pool
        .get()
        .await
        .map_err(|e| TestSetupError::SetupFailed(format!("Failed to get test client: {e}")))?;

    apply_migrations(&mut client)
        .await
        .map_err(|e| TestSetupError::SetupFailed(format!("Failed to apply test migrations: {e}")))
}

async fn clear_test_data(pool: &Pool) -> Result<(), TestSetupError> {
    let client = pool
        .get()
        .await
        .map_err(|e| TestSetupError::SetupFailed(format!("Failed to get test client: {e}")))?;

    client
        .batch_execute(
            "
            TRUNCATE TABLE songs, albums, artists CASCADE;
            ",
        )
        .await
        .map_err(|e| TestSetupError::SetupFailed(format!("Failed to clear test data: {e}")))
}

#[macro_export]
macro_rules! get_test_pool {
    () => {
        match $crate::tests::test_helpers::create_test_pool().await {
            | Ok(pool) => pool,
            | Err($crate::tests::test_helpers::TestSetupError::MissingDatabaseUrl) => {
                eprintln!("Skipping test — TEST_DATABASE_URL or DATABASE_URL not set");
                return;
            },
            | Err($crate::tests::test_helpers::TestSetupError::SetupFailed(e)) => {
                panic!("Test database setup failed: {e}");
            },
        }
    };
}
