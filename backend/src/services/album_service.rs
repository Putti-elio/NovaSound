use chrono::NaiveTime;
use deadpool_postgres::Pool;
use function_name::named;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::album_model::{Album, CreateAlbum, UpdateAlbum};
use crate::models::song_model::AlbumType;
use crate::utils::log_and_context_error;

#[named]
fn map_album(album: clorinde::queries::albums::Album) -> AppResult<Album> {
    let total_duration = u32::try_from(album.total_duration).map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Invalid album total_duration value in DB",
            file!(),
            function_name!(),
        ))
    })?;

    let release_date = album
        .release_date
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive()));
    let album_type = album.album_type.parse().unwrap_or(AlbumType::Album);

    Ok(Album {
        id: album.id,
        name: album.name,
        total_duration,
        release_date,
        artist_id: album.artist_id,
        image_path: album.image_path,
        album_type,
    })
}

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

    let albums = clorinde::queries::albums::get_all_albums()
        .bind(&client)
        .all()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query albums",
                file!(),
                function_name!(),
            ))
        })?;

    let albums = albums
        .into_iter()
        .map(map_album)
        .collect::<AppResult<Vec<_>>>()?;

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

    let album = clorinde::queries::albums::get_album_by_id()
        .bind(&client, &id)
        .opt()
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

    map_album(album)
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

    let albums = clorinde::queries::albums::get_albums_by_artist()
        .bind(&client, &artist_id)
        .all()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to query albums by artist",
                file!(),
                function_name!(),
            ))
        })?;

    let albums = albums
        .into_iter()
        .map(map_album)
        .collect::<AppResult<Vec<_>>>()?;

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

    let artist_exists = clorinde::queries::artists::check_artist_by_id()
        .bind(&client, &album.artist_id)
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
            album.artist_id
        )));
    }

    let existing = clorinde::queries::albums::check_album_by_name_and_artist()
        .bind(&client, &album.name, &album.artist_id)
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
    let album_type_str = album_type.as_ref();

    clorinde::queries::albums::insert_album()
        .bind(
            &client,
            &id,
            &album.name,
            &release_date_timestamp,
            &album.artist_id,
            &image_path,
            &album_type_str,
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

    let existing = clorinde::queries::albums::check_album_by_id()
        .bind(&client, &id)
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

    if !existing {
        return Err(AppError::NotFound(format!(
            "Album with id '{}' not found",
            id
        )));
    }

    if let Some(ref name) = album.name
        && name.trim().is_empty()
    {
        return Err(AppError::Validation(
            "Album name cannot be empty".to_string(),
        ));
    }

    if let Some(ref artist_id) = album.artist_id {
        let artist_exists = clorinde::queries::artists::check_artist_by_id()
            .bind(&client, &artist_id)
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
                artist_id
            )));
        }
    }

    if album.name.is_none() && album.release_date.is_none() && album.artist_id.is_none() {
        return Ok(());
    }

    let release_date_timestamp = album
        .release_date
        .map(|d| d.and_time(NaiveTime::MIN).and_utc().timestamp());

    let total_duration: Option<i32> = None;
    let image_path: Option<String> = None;
    let album_type: Option<String> = None;

    clorinde::queries::albums::update_album_partial()
        .bind(
            &client,
            &album.name,
            &release_date_timestamp,
            &album.artist_id,
            &total_duration,
            &image_path,
            &album_type,
            &id,
        )
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

    let rows_deleted = clorinde::queries::albums::delete_album()
        .bind(&client, &id)
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

    let total_duration_i64 = clorinde::queries::albums::calc_album_duration()
        .bind(&client, &album_id)
        .one()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to calculate album duration",
                file!(),
                function_name!(),
            ))
        })?;

    let total_duration = i32::try_from(total_duration_i64).map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Album duration is out of range",
            file!(),
            function_name!(),
        ))
    })?;

    clorinde::queries::albums::update_album_duration()
        .bind(&client, &total_duration, &album_id)
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
