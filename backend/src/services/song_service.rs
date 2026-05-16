use chrono::NaiveTime;
use deadpool_postgres::Pool;
use function_name::named;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::song_model::{AlbumType, CreateSong, Song, UpdateSong};
use crate::utils::log_and_context_error;

const STANDALONE_COLLECTION_SUFFIX: &str = "Standalone Collection";

#[named]
pub async fn get_all_songs(pool: &Pool) -> AppResult<Vec<Song>> {
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
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
            FROM songs ORDER BY track_number",
            &[],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query songs",
                file!(),
                function_name!(),
            ))
        })?;

    let songs = rows
        .iter()
        .map(|row| {
            let duration_i32: i32 = row.get(2);
            let release_date_timestamp: Option<i64> = row.get(5);
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            Song {
                id: row.get(0),
                name: row.get(1),
                duration: duration_i32 as u32,
                artist_id: row.get(3),
                album_id: row.get(4),
                release_date,
                track_number: row.get(6),
                image_path: row.get(7),
            }
        })
        .collect();

    Ok(songs)
}

#[named]
pub async fn get_song_by_id(pool: &Pool, id: &str) -> AppResult<Song> {
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
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
             FROM songs WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get song",
                file!(),
                function_name!(),
            ))
        })?
        .ok_or_else(|| AppError::NotFound(format!("Song with id '{}' not found", id)))?;

    let duration_i32: i32 = row.get(2);
    let release_date_timestamp: Option<i64> = row.get(5);
    let release_date = release_date_timestamp
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

    Ok(Song {
        id: row.get(0),
        name: row.get(1),
        duration: duration_i32 as u32,
        artist_id: row.get(3),
        album_id: row.get(4),
        release_date,
        track_number: row.get(6),
        image_path: row.get(7),
    })
}

#[named]
pub async fn get_songs_by_artist(pool: &Pool, artist_id: &str) -> AppResult<Vec<Song>> {
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
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
            FROM songs WHERE artist_id = $1 ORDER BY track_number",
            &[&artist_id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query songs by artist",
                file!(),
                function_name!(),
            ))
        })?;

    let songs = rows
        .iter()
        .map(|row| {
            let duration_i32: i32 = row.get(2);
            let release_date_timestamp: Option<i64> = row.get(5);
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            Song {
                id: row.get(0),
                name: row.get(1),
                duration: duration_i32 as u32,
                artist_id: row.get(3),
                album_id: row.get(4),
                release_date,
                track_number: row.get(6),
                image_path: row.get(7),
            }
        })
        .collect();

    Ok(songs)
}

#[named]
pub async fn get_songs_by_album(pool: &Pool, album_id: &str) -> AppResult<Vec<Song>> {
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
            "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path
            FROM songs WHERE album_id = $1 ORDER BY track_number",
            &[&album_id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query songs by album",
                file!(),
                function_name!(),
            ))
        })?;

    let songs = rows
        .iter()
        .map(|row| {
            let duration_i32: i32 = row.get(2);
            let release_date_timestamp: Option<i64> = row.get(5);
            let release_date = release_date_timestamp
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

            Song {
                id: row.get(0),
                name: row.get(1),
                duration: duration_i32 as u32,
                artist_id: row.get(3),
                album_id: row.get(4),
                release_date,
                track_number: row.get(6),
                image_path: row.get(7),
            }
        })
        .collect();

    Ok(songs)
}

#[named]
pub async fn create_song(pool: &Pool, song: CreateSong) -> AppResult<String> {
    if song.name.trim().is_empty() {
        return Err(AppError::Validation(
            "Song name cannot be empty".to_string(),
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
        .query_one("SELECT 1 FROM artists WHERE id = $1", &[&song.artist_id])
        .await
        .is_ok();

    if !artist_exists {
        return Err(AppError::Validation(format!(
            "Artist with id '{}' does not exist",
            song.artist_id
        )));
    }

    let mut album_id = song.album_id.clone();
    let image_path: Option<String>;

    if let Some(ref alb_id) = album_id {
        let album_exists: bool = client
            .query_one("SELECT 1 FROM albums WHERE id = $1", &[alb_id])
            .await
            .is_ok();

        if !album_exists {
            return Err(AppError::Validation(format!(
                "Album with id '{}' does not exist",
                alb_id
            )));
        }

        let album_image: Option<String> = client
            .query_one("SELECT image_path FROM albums WHERE id = $1", &[alb_id])
            .await
            .ok()
            .map(|row| row.get(0));
        image_path = album_image;
    } else {
        let standalone_id = get_or_create_standalone_collection(pool, &song.artist_id).await?;
        album_id = Some(standalone_id.clone());

        let artist_name: String = client
            .query_one("SELECT name FROM artists WHERE id = $1", &[&song.artist_id])
            .await
            .map_err(|err| {
                AppError::Internal(log_and_context_error(
                    err,
                    "Failed to get artist name",
                    file!(),
                    function_name!(),
                ))
            })?
            .get(0);

        image_path = Some(format!(
            "/images/artists/{}/{}/{}",
            artist_name.replace(' ', "_"),
            STANDALONE_COLLECTION_SUFFIX.replace(' ', "_"),
            standalone_id
        ));
    }

    let id = Uuid::new_v4().to_string();

    let release_date_timestamp = song
        .release_date
        .map(|d| d.and_time(NaiveTime::MIN).and_utc().timestamp());

    client
        .execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
        &id as &(dyn tokio_postgres::types::ToSql + Sync),
        &song.name,
                &(song.duration as i32),
                &song.artist_id,
                &album_id,
                &release_date_timestamp,
                &song.track_number,
                &image_path,
            ],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to create song",
                file!(),
                function_name!(),
            ))
        })?;

    if let Some(ref alb_id) = album_id {
        update_album_duration_and_type(pool, alb_id).await?;
    }

    Ok(id)
}

#[named]
pub async fn update_song(pool: &Pool, id: &str, song: UpdateSong) -> AppResult<()> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let existing: bool = client
        .query_one("SELECT 1 FROM songs WHERE id = $1", &[&id])
        .await
        .is_ok();

    if !existing {
        return Err(AppError::NotFound(format!(
            "Song with id '{}' not found",
            id
        )));
    }

    let mut set_clauses = Vec::new();
    let mut param_idx = 1;
    let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Send + Sync>> = Vec::new();

    if let Some(name) = song.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Song name cannot be empty".to_string(),
            ));
        }
        set_clauses.push(format!("name = ${}", param_idx));
        params.push(Box::new(name));
        param_idx += 1;
    }

    if let Some(duration) = song.duration {
        set_clauses.push(format!("duration = ${}", param_idx));
        params.push(Box::new(duration as i32));
        param_idx += 1;
    }

    if let Some(release_date) = song.release_date {
        let timestamp = release_date.and_time(NaiveTime::MIN).and_utc().timestamp();
        set_clauses.push(format!("release_date = ${}", param_idx));
        params.push(Box::new(timestamp));
        param_idx += 1;
    }

    if let Some(track_number) = song.track_number {
        set_clauses.push(format!("track_number = ${}", param_idx));
        params.push(Box::new(track_number));
        param_idx += 1;
    }

    if set_clauses.is_empty() {
        return Ok(());
    }

    set_clauses.push(format!("id = ${}", param_idx));
    params.push(Box::new(id.to_string()));

    let query = format!(
        "UPDATE songs SET {} WHERE id = ${}",
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
                "Failed to update song",
                file!(),
                function_name!(),
            ))
        })?;

    let album_id: Option<String> = client
        .query_one("SELECT album_id FROM songs WHERE id = $1", &[&id])
        .await
        .ok()
        .and_then(|row| row.get(0));

    if let Some(alb_id) = album_id {
        update_album_duration_and_type(pool, &alb_id).await?;
    }

    Ok(())
}

#[named]
pub async fn delete_song(pool: &Pool, id: &str) -> AppResult<()> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let album_id: Option<String> = client
        .query_one("SELECT album_id FROM songs WHERE id = $1", &[&id])
        .await
        .ok()
        .and_then(|row| row.get(0));

    let rows_deleted = client
        .execute("DELETE FROM songs WHERE id = $1", &[&id])
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to delete song",
                file!(),
                function_name!(),
            ))
        })?;

    if rows_deleted == 0 {
        return Err(AppError::NotFound(format!(
            "Song with id '{}' not found",
            id
        )));
    }

    if let Some(alb_id) = album_id {
        update_album_duration_and_type(pool, &alb_id).await?;
    }

    Ok(())
}

#[named]
pub async fn get_or_create_standalone_collection(
    pool: &Pool,
    artist_id: &str,
) -> AppResult<String> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let artist_name: String = client
        .query_one("SELECT name FROM artists WHERE id = $1", &[&artist_id])
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get artist name",
                file!(),
                function_name!(),
            ))
        })?
        .get(0);

    let collection_name = format!("{} {}", artist_name, STANDALONE_COLLECTION_SUFFIX);

    let existing: Option<String> = client
        .query_one(
            "SELECT id FROM albums WHERE artist_id = $1 AND album_type = $2",
            &[
                &artist_id,
                &AlbumType::StandaloneCollection.as_ref()
                    as &(dyn tokio_postgres::types::ToSql + Sync),
            ],
        )
        .await
        .ok()
        .map(|row| row.get(0));

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    let image_path = format!(
        "/images/artists/{}/{}",
        artist_name.replace(' ', "_"),
        STANDALONE_COLLECTION_SUFFIX.replace(' ', "_")
    );

    client
        .execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type)
            VALUES ($1, $2, 0, $3, $4, $5)",
            &[
                &id as &(dyn tokio_postgres::types::ToSql + Sync),
                &collection_name,
                &artist_id,
                &image_path,
                &AlbumType::StandaloneCollection.as_ref()
                    as &(dyn tokio_postgres::types::ToSql + Sync),
            ],
        )
        .await
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
pub async fn update_album_duration_and_type(pool: &Pool, album_id: &str) -> AppResult<()> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let album_type: String = client
        .query_one("SELECT album_type FROM albums WHERE id = $1", &[&album_id])
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get album type",
                file!(),
                function_name!(),
            ))
        })?
        .get(0);

    if album_type == AlbumType::StandaloneCollection.as_ref() {
        return Ok(());
    }

    let row = client
        .query_one(
            "SELECT COUNT(*), COALESCE(SUM(duration), 0) FROM songs WHERE album_id = $1",
            &[&album_id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to calculate album stats",
                file!(),
                function_name!(),
            ))
        })?;

    let song_count: i64 = row.get(0);
    let total_duration: i64 = row.get(1);

    let new_type = determine_album_type(song_count as i32, total_duration as u32);

    let total_duration_i32 = total_duration as i32;
    client
        .execute(
            "UPDATE albums SET total_duration = $1, album_type = $2 WHERE id = $3",
            &[
                &total_duration_i32 as &(dyn tokio_postgres::types::ToSql + Sync),
                &new_type.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync),
                &album_id,
            ],
        )
        .await
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

#[must_use]
pub fn determine_album_type(song_count: i32, total_duration: u32) -> AlbumType {
    if song_count >= 7 || total_duration >= 1800 {
        AlbumType::Album
    } else if (4..=6).contains(&song_count) || (900..1800).contains(&total_duration) {
        AlbumType::Ep
    } else {
        AlbumType::Single
    }
}
