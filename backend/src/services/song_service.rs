use chrono::NaiveTime;
use deadpool_postgres::{Client, Pool};
use function_name::named;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::song_model::{AlbumType, CreateSong, Song, UpdateSong};
use crate::utils::log_and_context_error;

const STANDALONE_COLLECTION_SUFFIX: &str = "Standalone Collection";

#[named]
fn map_song(song: clorinde::queries::songs::Song) -> AppResult<Song> {
    let duration = u32::try_from(song.duration).map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Invalid song duration value in DB",
            file!(),
            function_name!(),
        ))
    })?;

    let release_date = song
        .release_date
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));

    Ok(Song {
        id: song.id,
        name: song.name,
        duration,
        artist_id: song.artist_id,
        album_id: song.album_id,
        release_date,
        track_number: song.track_number,
        image_path: song.image_path,
    })
}

#[named]
async fn resolve_song_album_and_image(
    pool: &Pool,
    client: &Client,
    song: &CreateSong,
) -> AppResult<(Option<String>, Option<String>)> {
    if let Some(ref album_id) = song.album_id {
        let album_exists = clorinde::queries::albums::check_album_by_id()
            .bind(client, album_id)
            .opt()
            .await
            .map_err(|err| {
                AppError::Internal(log_and_context_error(
                    err,
                    "Failed to check album existence",
                    file!(),
                    function_name!(),
                ))
            })?
            .is_some();

        if !album_exists {
            return Err(AppError::Validation(format!(
                "Album with id '{}' does not exist",
                album_id
            )));
        }

        let image_path = clorinde::queries::albums::get_album_image_path()
            .bind(client, album_id)
            .one()
            .await
            .map_err(|err| {
                AppError::Internal(log_and_context_error(
                    err,
                    "Failed to get album image path",
                    file!(),
                    function_name!(),
                ))
            })?;

        return Ok((Some(album_id.clone()), image_path));
    }

    let standalone_id = get_or_create_standalone_collection(pool, &song.artist_id).await?;
    let artist_name = clorinde::queries::artists::get_artist_name_by_id()
        .bind(client, &song.artist_id)
        .one()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get artist name",
                file!(),
                function_name!(),
            ))
        })?;

    let image_path = Some(format!(
        "/images/artists/{}/{}/{}",
        artist_name.replace(' ', "_"),
        STANDALONE_COLLECTION_SUFFIX.replace(' ', "_"),
        standalone_id
    ));

    Ok((Some(standalone_id), image_path))
}

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

    let songs = clorinde::queries::songs::get_all_songs()
        .bind(&client)
        .all()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query songs",
                file!(),
                function_name!(),
            ))
        })?;

    let songs = songs
        .into_iter()
        .map(map_song)
        .collect::<AppResult<Vec<_>>>()?;

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

    let song = clorinde::queries::songs::get_song_by_id()
        .bind(&client, &id)
        .opt()
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

    map_song(song)
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

    let songs = clorinde::queries::songs::get_songs_by_artist()
        .bind(&client, &artist_id)
        .all()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query songs by artist",
                file!(),
                function_name!(),
            ))
        })?;

    let songs = songs
        .into_iter()
        .map(map_song)
        .collect::<AppResult<Vec<_>>>()?;

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

    let songs = clorinde::queries::songs::get_songs_by_album()
        .bind(&client, &album_id)
        .all()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query songs by album",
                file!(),
                function_name!(),
            ))
        })?;

    let songs = songs
        .into_iter()
        .map(map_song)
        .collect::<AppResult<Vec<_>>>()?;

    Ok(songs)
}

#[named]
pub async fn create_song(pool: &Pool, song: CreateSong) -> AppResult<Song> {
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

    let artist_exists = clorinde::queries::artists::check_artist_by_id()
        .bind(&client, &song.artist_id)
        .opt()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to check artist existence",
                file!(),
                function_name!(),
            ))
        })?
        .is_some();

    if !artist_exists {
        return Err(AppError::Validation(format!(
            "Artist with id '{}' does not exist",
            song.artist_id
        )));
    }

    let (album_id, image_path) = resolve_song_album_and_image(pool, &client, &song).await?;

    let id = Uuid::new_v4().to_string();

    let release_date_timestamp = song
        .release_date
        .map(|d| d.and_time(NaiveTime::MIN).and_utc().timestamp());

    let duration_i32 = i32::try_from(song.duration)
        .map_err(|_| AppError::Validation("Song duration is too large".to_string()))?;

    clorinde::queries::songs::insert_song()
        .bind(
            &client,
            &id,
            &song.name,
            &duration_i32,
            &song.artist_id,
            &album_id,
            &release_date_timestamp,
            &song.track_number,
            &image_path,
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

    Ok(Song {
        id,
        name: song.name,
        duration: song.duration,
        artist_id: song.artist_id,
        album_id,
        release_date: song.release_date,
        track_number: song.track_number,
        image_path,
    })
}

#[named]
pub async fn update_song(pool: &Pool, id: &str, song: UpdateSong) -> AppResult<Song> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let existing_song = clorinde::queries::songs::get_song_by_id()
        .bind(&client, &id)
        .opt()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to check song existence",
                file!(),
                function_name!(),
            ))
        })?;

    let existing_song = existing_song
        .map(map_song)
        .transpose()?
        .ok_or_else(|| AppError::NotFound(format!("Song with id '{}' not found", id)))?;

    if let Some(ref name) = song.name
        && name.trim().is_empty()
    {
        return Err(AppError::Validation(
            "Song name cannot be empty".to_string(),
        ));
    }

    if song.name.is_none()
        && song.duration.is_none()
        && song.release_date.is_none()
        && song.track_number.is_none()
    {
        return Ok(existing_song);
    }

    let duration_i32 = song
        .duration
        .map(|d| {
            i32::try_from(d)
                .map_err(|_| AppError::Validation("Song duration is too large".to_string()))
        })
        .transpose()?;
    let release_date_timestamp = song
        .release_date
        .map(|d| d.and_time(NaiveTime::MIN).and_utc().timestamp());

    let artist_id: Option<String> = None;
    let album_id: Option<String> = None;
    let image_path: Option<String> = None;

    clorinde::queries::songs::update_song_partial()
        .bind(
            &client,
            &song.name,
            &duration_i32,
            &artist_id,
            &album_id,
            &release_date_timestamp,
            &song.track_number,
            &image_path,
            &id,
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to update song",
                file!(),
                function_name!(),
            ))
        })?;

    let album_id = clorinde::queries::songs::get_song_album_id()
        .bind(&client, &id)
        .one()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get song album id",
                file!(),
                function_name!(),
            ))
        })?;

    if let Some(alb_id) = album_id {
        update_album_duration_and_type(pool, &alb_id).await?;
    }

    Ok(Song {
        id: existing_song.id,
        name: song.name.unwrap_or(existing_song.name),
        duration: song.duration.unwrap_or(existing_song.duration),
        artist_id: existing_song.artist_id,
        album_id: existing_song.album_id,
        release_date: song.release_date.or(existing_song.release_date),
        track_number: song.track_number.or(existing_song.track_number),
        image_path: existing_song.image_path,
    })
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

    let album_id = clorinde::queries::songs::get_song_album_id()
        .bind(&client, &id)
        .one()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get song album id",
                file!(),
                function_name!(),
            ))
        })?;

    let rows_deleted = clorinde::queries::songs::delete_song()
        .bind(&client, &id)
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

    let artist_name = clorinde::queries::artists::get_artist_name_by_id()
        .bind(&client, &artist_id)
        .one()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get artist name",
                file!(),
                function_name!(),
            ))
        })?;

    let collection_name = format!("{} {}", artist_name, STANDALONE_COLLECTION_SUFFIX);

    let standalone_type = AlbumType::StandaloneCollection.as_ref();
    let existing = clorinde::queries::albums::get_standalone_collection_id()
        .bind(&client, &artist_id, &standalone_type)
        .opt()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get standalone collection",
                file!(),
                function_name!(),
            ))
        })?
        .flatten();

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    let image_path = format!(
        "/images/artists/{}/{}",
        artist_name.replace(' ', "_"),
        STANDALONE_COLLECTION_SUFFIX.replace(' ', "_")
    );

    clorinde::queries::albums::insert_standalone_collection()
        .bind(
            &client,
            &id,
            &collection_name,
            &artist_id,
            &image_path,
            &standalone_type,
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

    let album_type = clorinde::queries::albums::get_album_type_by_id()
        .bind(&client, &album_id)
        .one()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get album type",
                file!(),
                function_name!(),
            ))
        })?;

    if album_type == AlbumType::StandaloneCollection.as_ref() {
        return Ok(());
    }

    let stats = clorinde::queries::albums::get_album_song_stats()
        .bind(&client, &album_id)
        .one()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to calculate album stats",
                file!(),
                function_name!(),
            ))
        })?;

    let song_count = stats.song_count;
    let total_duration = stats.total_duration;

    let song_count_i32 = i32::try_from(song_count).map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Album song_count is out of range",
            file!(),
            function_name!(),
        ))
    })?;
    let total_duration_u32 = u32::try_from(total_duration).map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Album total_duration is out of range",
            file!(),
            function_name!(),
        ))
    })?;
    let new_type = determine_album_type(song_count_i32, total_duration_u32);

    let total_duration_db_i32 = i32::try_from(total_duration).map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Album total_duration is out of range",
            file!(),
            function_name!(),
        ))
    })?;
    let new_type_str = new_type.as_ref();
    clorinde::queries::albums::update_album_duration_and_type()
        .bind(&client, &total_duration_db_i32, &new_type_str, &album_id)
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
