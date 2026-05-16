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

    let client = pool.get().await.map_err(|err| {
        error!(
            "Database couldn't be initialized: {}. At {}::{}",
            e,
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

    client
        .batch_execute(
            "
            CREATE TABLE IF NOT EXISTS artists (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                image_path TEXT
            );

        CREATE TABLE IF NOT EXISTS albums (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            total_duration INTEGER DEFAULT 0,
            release_date BIGINT,
            artist_id TEXT NOT NULL,
            image_path TEXT,
            album_type TEXT DEFAULT 'ALBUM',
            FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS songs (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            duration INTEGER,
            artist_id TEXT NOT NULL,
            album_id TEXT,
            release_date BIGINT,
                track_number INTEGER,
                image_path TEXT,
                FOREIGN KEY (artist_id) REFERENCES artists(id),
                FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE SET NULL
            );

            CREATE INDEX IF NOT EXISTS idx_songs_album_id ON songs(album_id);
            CREATE INDEX IF NOT EXISTS idx_songs_artist_id ON songs(artist_id);
            ",
        )
        .await
        .map_err(|err| {
            error!(
                "Failed to initialise the database and to create tables: {}. At {}::{}",
                err,
                file!(),
                function_name!()
            );
            crate::errors::AppError::Internal(crate::utils::log_and_context_error(
                err,
                "Failed to initialise the database and to create tables",
                file!(),
                function_name!(),
            ))
        })?;

    info!("Tables created successfully!");
    Ok(pool)
}
