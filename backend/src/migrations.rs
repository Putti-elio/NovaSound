use std::{fs, path::PathBuf};

use function_name::named;
use log::info;
use tokio_postgres::Client;

use crate::create_error;
use crate::errors::{AppError, AppResult};

struct Migration {
    version: String,
    sql: String,
}

pub async fn apply_migrations(client: &mut Client) -> AppResult<()> {
    ensure_migration_table(client).await?;

    for migration in load_migrations()? {
        let already_applied = is_migration_applied(client, &migration.version).await?;

        if already_applied {
            continue;
        }

        apply_single_migration(client, &migration.version, &migration.sql).await?;
    }

    info!("Database migrations are up to date.");
    Ok(())
}

#[named]
fn load_migrations() -> AppResult<Vec<Migration>> {
    let migrations_dir = PathBuf::from("database/migrations");
    let entries = fs::read_dir(&migrations_dir)
        .map_err(|err| create_error!(err, "Failed to read migrations directory"))?;

    let mut migrations = Vec::new();
    for entry in entries {
        let entry =
            entry.map_err(|err| create_error!(err, "Failed to read migration directory entry"))?;

        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("sql") {
            continue;
        }

        let version = path
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid migration file name")))?
            .to_string();
        let sql = fs::read_to_string(&path)
            .map_err(|err| create_error!(err, "Failed to read migration file"))?;

        migrations.push(Migration { version, sql });
    }

    migrations.sort_by(|left, right| left.version.cmp(&right.version));
    Ok(migrations)
}

#[named]
pub async fn reset_database(client: &Client) -> AppResult<()> {
    client
        .batch_execute(
            "
            DROP TABLE IF EXISTS songs;
            DROP TABLE IF EXISTS albums;
            DROP TABLE IF EXISTS artists CASCADE;
            DROP TABLE IF EXISTS schema_migrations;
            ",
        )
        .await
        .map_err(|err| create_error!(err, "Failed to reset database schema"))?;

    info!("Database schema reset successfully.");
    Ok(())
}

#[named]
async fn ensure_migration_table(client: &Client) -> AppResult<()> {
    client
        .batch_execute(
            "
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
            ",
        )
        .await
        .map_err(|err| create_error!(err, "Failed to ensure schema_migrations table exists"))?;

    Ok(())
}

#[named]
async fn is_migration_applied(client: &Client, version: &str) -> AppResult<bool> {
    let row = client
        .query_opt(
            "SELECT 1 FROM schema_migrations WHERE version = $1",
            &[&version],
        )
        .await
        .map_err(|err| create_error!(err, "Failed to check migration status"))?;

    Ok(row.is_some())
}

#[named]
async fn apply_single_migration(client: &mut Client, version: &str, sql: &str) -> AppResult<()> {
    info!("Applying migration {}...", version);

    let transaction = client
        .transaction()
        .await
        .map_err(|err| create_error!(err, "Failed to start migration transaction"))?;

    transaction
        .batch_execute(sql)
        .await
        .map_err(|err| create_error!(err, "Failed to execute migration"))?;

    transaction
        .execute(
            "INSERT INTO schema_migrations (version) VALUES ($1)",
            &[&version],
        )
        .await
        .map_err(|err| create_error!(err, "Failed to record applied migration"))?;

    transaction
        .commit()
        .await
        .map_err(|err| create_error!(err, "Failed to commit migration transaction"))?;

    info!("Migration {} applied successfully.", version);
    Ok(())
}
