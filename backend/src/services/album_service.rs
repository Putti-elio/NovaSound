use chrono::NaiveTime;
use deadpool_postgres::Pool;
use function_name::named;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::album_model::{Album, CreateAlbum, UpdateAlbum};
use crate::models::song_model::AlbumType;
use crate::utils::log_and_context_error;

#[named]
pub async fn get_all_albums(pool: &Pool) -> AppResult<Vec<Album>> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let rows = client
        .query(
            "SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums",
            &[],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
        "Failed to query albums",
            file!(),
            function_name!(),
        ))
    })?;

    let albums = rows
        .iter()
        .map(|row| {
            let total_duration_i32: i32 = row.get(2);
            let release_date_timestamp: Option<i64> = row.get(3);
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            let album_type_str: String = row.get(6);
            let album_type = album_type_str.parse().unwrap_or(AlbumType::Album);

            Album {
                id: row.get(0),
                name: row.get(1),
                total_duration: total_duration_i32 as u32,
                release_date,
                artist_id: row.get(4),
                image_path: row.get(5),
                album_type,
            }
        })
        .collect();

    Ok(albums)
}

#[named]
pub async fn get_album_by_id(pool: &Pool, id: &str) -> AppResult<Album> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let row = client
        .query_opt(
            "SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
        "Failed to get album",
            file!(),
            function_name!(),
        ))
    })?
    .ok_or_else(|| AppError::NotFound(format!("Album with id '{}' not found", id)))?;

    let total_duration_i32: i32 = row.get(2);
    let release_date_timestamp: Option<i64> = row.get(3);
    let release_date = release_date_timestamp
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

    let album_type_str: String = row.get(6);
    let album_type = album_type_str.parse().unwrap_or(AlbumType::Album);

    Ok(Album {
        id: row.get(0),
        name: row.get(1),
        total_duration: total_duration_i32 as u32,
        release_date,
        artist_id: row.get(4),
        image_path: row.get(5),
        album_type,
    })
}

#[named]
pub async fn get_albums_by_artist(pool: &Pool, artist_id: &str) -> AppResult<Vec<Album>> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let rows = client
        .query(
            "SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE artist_id = $1",
            &[&artist_id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
        "Failed to query albums by artist",
            file!(),
            function_name!(),
        ))
    })?;

    let albums = rows
        .iter()
        .map(|row| {
            let total_duration_i32: i32 = row.get(2);
            let release_date_timestamp: Option<i64> = row.get(3);
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            let album_type_str: String = row.get(6);
            let album_type = album_type_str.parse().unwrap_or(AlbumType::Album);

            Album {
                id: row.get(0),
                name: row.get(1),
                total_duration: total_duration_i32 as u32,
                release_date,
                artist_id: row.get(4),
                image_path: row.get(5),
                album_type,
            }
        })
        .collect();

    Ok(albums)
}

#[named]
pub async fn create_album(pool: &Pool, album: CreateAlbum) -> AppResult<String> {
    if album.name.trim().is_empty() {
        return Err(AppError::Validation(
            "Album name cannot be empty".to_string(),
        ));
    }

    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let artist_exists: bool = client
        .query_one("SELECT 1 FROM artists WHERE id = $1", &[&album.artist_id])
        .await
        .is_ok();

    if !artist_exists {
        return Err(AppError::Validation(format!(
            "Artist with id '{}' does not exist",
            album.artist_id
        )));
    }

    let existing: bool = client
        .query_one(
            "SELECT 1 FROM albums WHERE name = $1 AND artist_id = $2",
            &[&album.name, &album.artist_id],
        )
        .await
        .is_ok();

    if existing {
        return Err(AppError::Validation(format!(
            "Album '{}' already exists for this artist",
            album.name
        )));
    }

    let id = Uuid::new_v4().to_string();
    let image_path = format!("/images/albums/{}", id);

    let release_date_timestamp = album
        .release_date
        .map(|d| d.and_time(NaiveTime::MIN).and_utc().timestamp());

    let album_type = album.album_type.unwrap_or(AlbumType::Album);

    client
        .execute(
            "INSERT INTO albums (id, name, total_duration, release_date, artist_id, image_path, album_type)
            VALUES ($1, $2, 0, $3, $4, $5, $6)",
            &[
                &id as &(dyn tokio_postgres::types::ToSql + Sync),
                &album.name,
                &release_date_timestamp,
                &album.artist_id,
                &image_path,
                &album_type.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync),
            ],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to create album",
                file!(),
                function_name!(),
            ))
        })?;

    Ok(id)
}

#[named]
pub async fn update_album(pool: &Pool, id: &str, album: UpdateAlbum) -> AppResult<()> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let existing: bool = client
        .query_one("SELECT 1 FROM albums WHERE id = $1", &[&id])
        .await
        .is_ok();

    if !existing {
        return Err(AppError::NotFound(format!(
            "Album with id '{}' not found",
            id
        )));
    }

    let mut set_clauses = Vec::new();
    let mut param_idx = 1;
    let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Send + Sync>> = Vec::new();

    if let Some(name) = album.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Album name cannot be empty".to_string(),
            ));
        }
        set_clauses.push(format!("name = ${}", param_idx));
        params.push(Box::new(name));
        param_idx += 1;
    }

    if let Some(release_date) = album.release_date {
        let timestamp = release_date.and_time(NaiveTime::MIN).and_utc().timestamp();
        set_clauses.push(format!("release_date = ${}", param_idx));
        params.push(Box::new(timestamp));
        param_idx += 1;
    }

    if let Some(artist_id) = album.artist_id {
        let artist_exists: bool = client
            .query_one("SELECT 1 FROM artists WHERE id = $1", &[&artist_id])
            .await
            .is_ok();

        if !artist_exists {
            return Err(AppError::Validation(format!(
                "Artist with id '{}' does not exist",
                artist_id
            )));
        }

        set_clauses.push(format!("artist_id = ${}", param_idx));
        params.push(Box::new(artist_id));
        param_idx += 1;
    }

    if set_clauses.is_empty() {
        return Ok(());
    }

    set_clauses.push(format!("id = ${}", param_idx));
    params.push(Box::new(id.to_string()));

    let query = format!(
        "UPDATE albums SET {} WHERE id = ${}",
        set_clauses.join(", "),
        param_idx
    );

    let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params
        .iter()
        .map(|p| p.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
        .collect();

    client
        .execute(&query, param_refs.as_slice())
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to update album",
                file!(),
                function_name!(),
            ))
        })?;

    Ok(())
}

#[named]
pub async fn delete_album(pool: &Pool, id: &str) -> AppResult<()> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let rows_deleted = client
        .execute("DELETE FROM albums WHERE id = $1", &[&id])
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to delete album",
                file!(),
                function_name!(),
            ))
        })?;

    if rows_deleted == 0 {
        return Err(AppError::NotFound(format!(
            "Album with id '{}' not found",
            id
        )));
    }

    Ok(())
}

#[named]
pub async fn update_album_duration(pool: &Pool, album_id: &str) -> AppResult<()> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let total_duration: i32 = client
        .query_one(
            "SELECT COALESCE(SUM(duration), 0) FROM songs WHERE album_id = $1",
            &[&album_id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to calculate album duration",
                file!(),
                function_name!(),
            ))
        })?
        .get::<_, i64>(0) as i32;

    client
        .execute(
            "UPDATE albums SET total_duration = $1 WHERE id = $2",
            &[&total_duration, &album_id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to update album duration",
                file!(),
                function_name!(),
            ))
        })?;

    Ok(())
}
