use chrono::NaiveTime;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::album_model::{Album, CreateAlbum, UpdateAlbum};
use crate::models::song_model::AlbumType;
use crate::utils::log_and_context_error;
use function_name::named;

#[named]
pub fn get_all_albums(conn: &Connection) -> AppResult<Vec<Album>> {
    let mut stmt = conn
        .prepare("SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums")
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare album query",
                file!(),
                function_name!(),
            )
        })?;

    let albums = stmt
        .query_map([], |row| {
            let release_date_timestamp: Option<i64> = row.get(3)?;
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            let album_type_str: String = row.get(6)?;
            let album_type = AlbumType::from_str(&album_type_str).unwrap_or(AlbumType::Album);

            Ok(Album {
                id: row.get(0)?,
                name: row.get(1)?,
                total_duration: row.get(2)?,
                release_date,
                artist_id: row.get(4)?,
                image_path: row.get(5)?,
                album_type,
            })
        })
        .map_err(|err| {
            log_and_context_error(err, "Failed to query albums", file!(), function_name!())
        })?;

    albums.collect::<Result<Vec<_>, _>>().map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to collect albums",
            file!(),
            function_name!(),
        ))
    })
}

#[named]
pub fn get_album_by_id(conn: &Connection, id: &str) -> AppResult<Album> {
    let mut stmt = conn
        .prepare("SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE id = ?1")
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare album query",
                file!(),
                function_name!(),
            )
        })?;

    stmt.query_row(params![id], |row| {
        let release_date_timestamp: Option<i64> = row.get(3)?;
        let release_date = release_date_timestamp
            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

        let album_type_str: String = row.get(6)?;
        let album_type = AlbumType::from_str(&album_type_str).unwrap_or(AlbumType::Album);

        Ok(Album {
            id: row.get(0)?,
            name: row.get(1)?,
            total_duration: row.get(2)?,
            release_date,
            artist_id: row.get(4)?,
            image_path: row.get(5)?,
            album_type,
        })
    })
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Album with id '{}' not found", id))
        }
        _ => AppError::Internal(log_and_context_error(
            err,
            "Failed to get album",
            file!(),
            function_name!(),
        )),
    })
}

#[named]
pub fn get_albums_by_artist(conn: &Connection, artist_id: &str) -> AppResult<Vec<Album>> {
    let mut stmt = conn
        .prepare("SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE artist_id = ?1")
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare album query",
                file!(),
                function_name!(),
            )
        })?;

    let albums = stmt
        .query_map(params![artist_id], |row| {
            let release_date_timestamp: Option<i64> = row.get(3)?;
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            let album_type_str: String = row.get(6)?;
            let album_type = AlbumType::from_str(&album_type_str).unwrap_or(AlbumType::Album);

            Ok(Album {
                id: row.get(0)?,
                name: row.get(1)?,
                total_duration: row.get(2)?,
                release_date,
                artist_id: row.get(4)?,
                image_path: row.get(5)?,
                album_type,
            })
        })
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to query albums by artist",
                file!(),
                function_name!(),
            )
        })?;

    albums.collect::<Result<Vec<_>, _>>().map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to collect albums",
            file!(),
            function_name!(),
        ))
    })
}

#[named]
pub fn create_album(conn: &Connection, album: CreateAlbum) -> AppResult<String> {
    if album.name.trim().is_empty() {
        return Err(AppError::Validation(
            "Album name cannot be empty".to_string(),
        ));
    }

    let artist_exists: bool = conn
        .query_row(
            "SELECT 1 FROM artists WHERE id = ?1",
            params![album.artist_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !artist_exists {
        return Err(AppError::Validation(format!(
            "Artist with id '{}' does not exist",
            album.artist_id
        )));
    }

    let existing: bool = conn
        .query_row(
            "SELECT 1 FROM albums WHERE name = ?1 AND artist_id = ?2",
            params![album.name, album.artist_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if existing {
        return Err(AppError::Validation(format!(
            "Album '{}' already exists for this artist",
            album.name
        )));
    }

    let id = Uuid::new_v4().to_string();
    let image_path = format!("/images/albums/{}", id);

    let release_date_timestamp = album.release_date.map(|d| {
        d.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp()
    });

    let album_type = album.album_type.unwrap_or(AlbumType::Album);

    conn.execute(
        "INSERT INTO albums (id, name, total_duration, release_date, artist_id, image_path, album_type) 
         VALUES (?1, ?2, 0, ?3, ?4, ?5, ?6)",
        params![
            &id,
            &album.name,
            release_date_timestamp,
            &album.artist_id,
            &image_path,
            album_type.as_str()
        ],
    )
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
pub fn update_album(conn: &Connection, id: &str, album: UpdateAlbum) -> AppResult<()> {
    let existing: bool = conn
        .query_row("SELECT 1 FROM albums WHERE id = ?1", params![id], |_| {
            Ok(true)
        })
        .unwrap_or(false);

    if !existing {
        return Err(AppError::NotFound(format!(
            "Album with id '{}' not found",
            id
        )));
    }

    let mut updates = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(name) = album.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Album name cannot be empty".to_string(),
            ));
        }
        updates.push("name = ?".to_string());
        params_vec.push(Box::new(name));
    }

    if let Some(release_date) = album.release_date {
        let timestamp = release_date
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp();
        updates.push("release_date = ?".to_string());
        params_vec.push(Box::new(timestamp));
    }

    if let Some(artist_id) = album.artist_id {
        let artist_exists: bool = conn
            .query_row(
                "SELECT 1 FROM artists WHERE id = ?1",
                params![&artist_id],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !artist_exists {
            return Err(AppError::Validation(format!(
                "Artist with id '{}' does not exist",
                artist_id
            )));
        }

        updates.push("artist_id = ?".to_string());
        params_vec.push(Box::new(artist_id));
    }

    if updates.is_empty() {
        return Ok(());
    }

    let query = format!("UPDATE albums SET {} WHERE id = ?", updates.join(", "));
    params_vec.push(Box::new(id.to_string()));

    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    conn.execute(&query, params_refs.as_slice())
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
pub fn delete_album(conn: &Connection, id: &str) -> AppResult<()> {
    let rows_deleted = conn
        .execute("DELETE FROM albums WHERE id = ?1", params![id])
        .map_err(|err| {
            log_and_context_error(err, "Failed to delete album", file!(), function_name!())
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
pub fn update_album_duration(conn: &Connection, album_id: &str) -> AppResult<()> {
    let total_duration: u32 = conn
        .query_row(
            "SELECT COALESCE(SUM(duration), 0) FROM songs WHERE album_id = ?1",
            params![album_id],
            |row| row.get(0),
        )
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to calculate album duration",
                file!(),
                function_name!(),
            )
        })?;

    conn.execute(
        "UPDATE albums SET total_duration = ?1 WHERE id = ?2",
        params![total_duration, album_id],
    )
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
