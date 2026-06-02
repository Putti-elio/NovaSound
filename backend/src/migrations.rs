use std::{fs, path::PathBuf};

use function_name::named;
use log::{error, info};
use tokio_postgres::Client;

use crate::errors::{AppError, AppResult};
use crate::utils::log_and_context_error;

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
    let entries = fs::read_dir(&migrations_dir).map_err(|err| {
        error!(
            "Failed to read migrations directory {}: {}. At {}::{}",
            migrations_dir.display(),
            err,
            file!(),
            function_name!()
        );
        AppError::Internal(log_and_context_error(
            err,
            "Failed to read migrations directory",
            file!(),
            function_name!(),
        ))
    })?;

    let mut migrations = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|err| {
            error!(
                "Failed to read migration directory entry: {}. At {}::{}",
                err,
                file!(),
                function_name!()
            );
            AppError::Internal(log_and_context_error(
                err,
                "Failed to read migration directory entry",
                file!(),
                function_name!(),
            ))
        })?;

        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("sql") {
            continue;
        }

        let version = path
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid migration file name")))?
            .to_string();
        let sql = fs::read_to_string(&path).map_err(|err| {
            error!(
                "Failed to read migration file {}: {}. At {}::{}",
                path.display(),
                err,
                file!(),
                function_name!()
            );
            AppError::Internal(log_and_context_error(
                err,
                "Failed to read migration file",
                file!(),
                function_name!(),
            ))
        })?;

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
        .map_err(|err| {
            error!(
                "Failed to reset database schema: {}. At {}::{}",
                err,
                file!(),
                function_name!()
            );
            AppError::Internal(log_and_context_error(
                err,
                "Failed to reset database schema",
                file!(),
                function_name!(),
            ))
        })?;

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
        .map_err(|err| {
            error!(
                "Failed to ensure schema_migrations table exists: {}. At {}::{}",
                err,
                file!(),
                function_name!()
            );
            AppError::Internal(log_and_context_error(
                err,
                "Failed to ensure schema_migrations table exists",
                file!(),
                function_name!(),
            ))
        })?;

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
        .map_err(|err| {
            error!(
                "Failed to check migration status for {}: {}. At {}::{}",
                version,
                err,
                file!(),
                function_name!()
            );
            AppError::Internal(log_and_context_error(
                err,
                "Failed to check migration status",
                file!(),
                function_name!(),
            ))
        })?;

    Ok(row.is_some())
}

#[named]
async fn apply_single_migration(client: &mut Client, version: &str, sql: &str) -> AppResult<()> {
    info!("Applying migration {}...", version);

    let transaction = client.transaction().await.map_err(|err| {
        error!(
            "Failed to start migration transaction for {}: {}. At {}::{}",
            version,
            err,
            file!(),
            function_name!()
        );
        AppError::Internal(log_and_context_error(
            err,
            "Failed to start migration transaction",
            file!(),
            function_name!(),
        ))
    })?;

    transaction.batch_execute(sql).await.map_err(|err| {
        error!(
            "Failed to execute migration {}: {}. At {}::{}",
            version,
            err,
            file!(),
            function_name!()
        );
        AppError::Internal(log_and_context_error(
            err,
            "Failed to execute migration",
            file!(),
            function_name!(),
        ))
    })?;

    transaction
        .execute(
            "INSERT INTO schema_migrations (version) VALUES ($1)",
            &[&version],
        )
        .await
        .map_err(|err| {
            error!(
                "Failed to record migration {}: {}. At {}::{}",
                version,
                err,
                file!(),
                function_name!()
            );
            AppError::Internal(log_and_context_error(
                err,
                "Failed to record applied migration",
                file!(),
                function_name!(),
            ))
        })?;

    transaction.commit().await.map_err(|err| {
        error!(
            "Failed to commit migration {}: {}. At {}::{}",
            version,
            err,
            file!(),
            function_name!()
        );
        AppError::Internal(log_and_context_error(
            err,
            "Failed to commit migration transaction",
            file!(),
            function_name!(),
        ))
    })?;

    info!("Migration {} applied successfully.", version);
    Ok(())
}
