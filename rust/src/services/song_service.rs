use chrono::NaiveTime;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::song_model::{AlbumType, CreateSong, Song, UpdateSong};
use crate::utils::log_and_context_error;
use function_name::named;

const STANDALONE_COLLECTION_SUFFIX: &str = "Standalone Collection";

#[named]
pub fn get_all_songs(conn: &Connection) -> AppResult<Vec<Song>> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path 
             FROM songs ORDER BY track_number",
        )
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare songs query",
                file!(),
                function_name!(),
            )
        })?;

    let songs = stmt
        .query_map([], |row| {
            let release_date_timestamp: Option<i64> = row.get(5)?;
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            Ok(Song {
                id: row.get(0)?,
                name: row.get(1)?,
                duration: row.get(2)?,
                artist_id: row.get(3)?,
                album_id: row.get(4)?,
                release_date,
                track_number: row.get(6)?,
                image_path: row.get(7)?,
            })
        })
        .map_err(|err| {
            log_and_context_error(err, "Failed to query songs", file!(), function_name!())
        })?;

    songs.collect::<Result<Vec<_>, _>>().map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to collect songs",
            file!(),
            function_name!(),
        ))
    })
}

#[named]
pub fn get_song_by_id(conn: &Connection, id: &str) -> AppResult<Song> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path 
             FROM songs WHERE id = ?1",
        )
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare song query",
                file!(),
                function_name!(),
            )
        })?;

    stmt.query_row(params![id], |row| {
        let release_date_timestamp: Option<i64> = row.get(5)?;
        let release_date = release_date_timestamp
            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

        Ok(Song {
            id: row.get(0)?,
            name: row.get(1)?,
            duration: row.get(2)?,
            artist_id: row.get(3)?,
            album_id: row.get(4)?,
            release_date,
            track_number: row.get(6)?,
            image_path: row.get(7)?,
        })
    })
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Song with id '{}' not found", id))
        }
        _ => AppError::Internal(log_and_context_error(
            err,
            "Failed to get song",
            file!(),
            function_name!(),
        )),
    })
}

#[named]
pub fn get_songs_by_artist(conn: &Connection, artist_id: &str) -> AppResult<Vec<Song>> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path 
             FROM songs WHERE artist_id = ?1 ORDER BY track_number",
        )
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare songs query",
                file!(),
                function_name!(),
            )
        })?;

    let songs = stmt
        .query_map(params![artist_id], |row| {
            let release_date_timestamp: Option<i64> = row.get(5)?;
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            Ok(Song {
                id: row.get(0)?,
                name: row.get(1)?,
                duration: row.get(2)?,
                artist_id: row.get(3)?,
                album_id: row.get(4)?,
                release_date,
                track_number: row.get(6)?,
                image_path: row.get(7)?,
            })
        })
        .map_err(|err| {
            log_and_context_error(err, "Failed to query songs", file!(), function_name!())
        })?;

    songs.collect::<Result<Vec<_>, _>>().map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to collect songs",
            file!(),
            function_name!(),
        ))
    })
}

#[named]
pub fn get_songs_by_album(conn: &Connection, album_id: &str) -> AppResult<Vec<Song>> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path 
             FROM songs WHERE album_id = ?1 ORDER BY track_number",
        )
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare songs query",
                file!(),
                function_name!(),
            )
        })?;

    let songs = stmt
        .query_map(params![album_id], |row| {
            let release_date_timestamp: Option<i64> = row.get(5)?;
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            Ok(Song {
                id: row.get(0)?,
                name: row.get(1)?,
                duration: row.get(2)?,
                artist_id: row.get(3)?,
                album_id: row.get(4)?,
                release_date,
                track_number: row.get(6)?,
                image_path: row.get(7)?,
            })
        })
        .map_err(|err| {
            log_and_context_error(err, "Failed to query songs", file!(), function_name!())
        })?;

    songs.collect::<Result<Vec<_>, _>>().map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to collect songs",
            file!(),
            function_name!(),
        ))
    })
}

#[named]
pub fn create_song(conn: &Connection, song: CreateSong) -> AppResult<String> {
    if song.name.trim().is_empty() {
        return Err(AppError::Validation(
            "Song name cannot be empty".to_string(),
        ));
    }

    let artist_exists: bool = conn
        .query_row(
            "SELECT 1 FROM artists WHERE id = ?1",
            params![song.artist_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !artist_exists {
        return Err(AppError::Validation(format!(
            "Artist with id '{}' does not exist",
            song.artist_id
        )));
    }

    let mut album_id = song.album_id.clone();
    let image_path: Option<String>;

    if let Some(ref alb_id) = album_id {
        let album_exists: bool = conn
            .query_row(
                "SELECT 1 FROM albums WHERE id = ?1",
                params![alb_id],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !album_exists {
            return Err(AppError::Validation(format!(
                "Album with id '{}' does not exist",
                alb_id
            )));
        }

        let album_image: Option<String> = conn
            .query_row(
                "SELECT image_path FROM albums WHERE id = ?1",
                params![alb_id],
                |row| row.get(0),
            )
            .ok();
        image_path = album_image;
    } else {
        let standalone_id = get_or_create_standalone_collection(conn, &song.artist_id)?;
        album_id = Some(standalone_id.clone());

        let artist_name: String = conn
            .query_row(
                "SELECT name FROM artists WHERE id = ?1",
                params![&song.artist_id],
                |row| row.get(0),
            )
            .map_err(|err| {
                AppError::Internal(log_and_context_error(
                    err,
                    "Failed to get artist name",
                    file!(),
                    function_name!(),
                ))
            })?;

        image_path = Some(format!(
            "/images/artists/{}/{}/{}",
            artist_name.replace(" ", "_"),
            STANDALONE_COLLECTION_SUFFIX.replace(" ", "_"),
            standalone_id
        ));
    }

    let id = Uuid::new_v4().to_string();

    let release_date_timestamp = song.release_date.map(|d| {
        d.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp()
    });

    conn.execute(
        "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            &id,
            &song.name,
            song.duration,
            &song.artist_id,
            &album_id,
            release_date_timestamp,
            song.track_number,
            &image_path
        ],
    )
    .map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to create song",
            file!(),
            function_name!(),
        ))
    })?;

    if let Some(ref alb_id) = album_id {
        update_album_duration_and_type(conn, alb_id)?;
    }

    Ok(id)
}

#[named]
pub fn update_song(conn: &Connection, id: &str, song: UpdateSong) -> AppResult<()> {
    let existing: bool = conn
        .query_row("SELECT 1 FROM songs WHERE id = ?1", params![id], |_| {
            Ok(true)
        })
        .unwrap_or(false);

    if !existing {
        return Err(AppError::NotFound(format!(
            "Song with id '{}' not found",
            id
        )));
    }

    let mut updates = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(name) = song.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Song name cannot be empty".to_string(),
            ));
        }
        updates.push("name = ?".to_string());
        params_vec.push(Box::new(name));
    }

    if let Some(duration) = song.duration {
        updates.push("duration = ?".to_string());
        params_vec.push(Box::new(duration));
    }

    if let Some(release_date) = song.release_date {
        let timestamp = release_date
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp();
        updates.push("release_date = ?".to_string());
        params_vec.push(Box::new(timestamp));
    }

    if let Some(track_number) = song.track_number {
        updates.push("track_number = ?".to_string());
        params_vec.push(Box::new(track_number));
    }

    if updates.is_empty() {
        return Ok(());
    }

    let query = format!("UPDATE songs SET {} WHERE id = ?", updates.join(", "));
    params_vec.push(Box::new(id.to_string()));

    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    conn.execute(&query, params_refs.as_slice())
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to update song",
                file!(),
                function_name!(),
            ))
        })?;

    let album_id: Option<String> = conn
        .query_row(
            "SELECT album_id FROM songs WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .ok()
        .flatten();

    if let Some(alb_id) = album_id {
        update_album_duration_and_type(conn, &alb_id)?;
    }

    Ok(())
}

#[named]
pub fn delete_song(conn: &Connection, id: &str) -> AppResult<()> {
    let album_id: Option<String> = conn
        .query_row(
            "SELECT album_id FROM songs WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .ok()
        .flatten();

    let rows_deleted = conn
        .execute("DELETE FROM songs WHERE id = ?1", params![id])
        .map_err(|err| {
            log_and_context_error(err, "Failed to delete song", file!(), function_name!())
        })?;

    if rows_deleted == 0 {
        return Err(AppError::NotFound(format!(
            "Song with id '{}' not found",
            id
        )));
    }

    if let Some(alb_id) = album_id {
        update_album_duration_and_type(conn, &alb_id)?;
    }

    Ok(())
}

#[named]
pub fn get_or_create_standalone_collection(
    conn: &Connection,
    artist_id: &str,
) -> AppResult<String> {
    let artist_name: String = conn
        .query_row(
            "SELECT name FROM artists WHERE id = ?1",
            params![artist_id],
            |row| row.get(0),
        )
        .map_err(|err| {
            log_and_context_error(err, "Failed to get artist name", file!(), function_name!())
        })?;

    let collection_name = format!("{} {}", artist_name, STANDALONE_COLLECTION_SUFFIX);

    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM albums WHERE artist_id = ?1 AND album_type = ?2",
            params![artist_id, AlbumType::StandaloneCollection.as_str()],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    let image_path = format!(
        "/images/artists/{}/{}",
        artist_name.replace(" ", "_"),
        STANDALONE_COLLECTION_SUFFIX.replace(" ", "_")
    );

    conn.execute(
        "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) 
         VALUES (?1, ?2, 0, ?3, ?4, ?5)",
        params![
            &id,
            &collection_name,
            artist_id,
            &image_path,
            AlbumType::StandaloneCollection.as_str()
        ],
    )
    .map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to create standalone collection",
            file!(),
            function_name!(),
        ))
    })?;

    Ok(id)
}

#[named]
pub fn update_album_duration_and_type(conn: &Connection, album_id: &str) -> AppResult<()> {
    let album_type: String = conn
        .query_row(
            "SELECT album_type FROM albums WHERE id = ?1",
            params![album_id],
            |row| row.get(0),
        )
        .map_err(|err| {
            log_and_context_error(err, "Failed to get album type", file!(), function_name!())
        })?;

    if album_type == AlbumType::StandaloneCollection.as_str() {
        return Ok(());
    }

    let (song_count, total_duration): (i32, u32) = conn
        .query_row(
            "SELECT COUNT(*), COALESCE(SUM(duration), 0) FROM songs WHERE album_id = ?1",
            params![album_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to calculate album stats",
                file!(),
                function_name!(),
            )
        })?;

    let new_type = determine_album_type(song_count, total_duration);

    conn.execute(
        "UPDATE albums SET total_duration = ?1, album_type = ?2 WHERE id = ?3",
        params![total_duration, new_type.as_str(), album_id],
    )
    .map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to update album duration and type",
            file!(),
            function_name!(),
        ))
    })?;

    Ok(())
}

pub fn determine_album_type(song_count: i32, total_duration: u32) -> AlbumType {
    if song_count >= 7 || total_duration >= 1800 {
        AlbumType::Album
    } else if (4..=6).contains(&song_count) || (900..1800).contains(&total_duration) {
        AlbumType::Ep
    } else {
        AlbumType::Single
    }
}
