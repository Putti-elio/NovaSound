use deadpool_postgres::Pool;
use function_name::named;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::artist_model::Artist;
use crate::utils::log_and_context_error;

fn map_artist(artist: clorinde::queries::artists::Artist) -> Artist {
    Artist {
        id: artist.id,
        name: artist.name,
        image_path: artist.image_path,
    }
}

#[named]
pub async fn get_all_artists(pool: &Pool) -> AppResult<Vec<Artist>> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let artists = clorinde::queries::artists::get_all_artists()
        .bind(&client)
        .all()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get artists",
                file!(),
                function_name!(),
            ))
        })?;

    Ok(artists.into_iter().map(map_artist).collect())
}

#[named]
pub async fn create_artist(pool: &Pool, name: &str) -> AppResult<()> {
    if name.trim().is_empty() {
        return Err(AppError::Validation(
            "Artist name cannot be empty".to_string(),
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

    let exists = clorinde::queries::artists::check_artist_by_name()
        .bind(&client, &name)
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

    if exists {
        return Err(AppError::Validation(format!(
            "Artist '{}' already exists",
            name
        )));
    }

    let id = Uuid::new_v4().to_string();
    let image_path = "/images/".to_owned() + name;

    clorinde::queries::artists::insert_artist()
        .bind(&client, &id, &name, &image_path)
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to insert artist",
                file!(),
                function_name!(),
            ))
        })?;

    Ok(())
}

#[named]
pub async fn get_artist(pool: &Pool, id: &String) -> AppResult<Artist> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let artist = clorinde::queries::artists::get_artist_by_id()
        .bind(&client, id)
        .opt()
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to get artist",
                file!(),
                function_name!(),
            ))
        })?
        .ok_or_else(|| AppError::NotFound(format!("Artist with id '{}' not found", id)))?;

    Ok(map_artist(artist))
}

#[named]
pub async fn update_artist(pool: &Pool, id: &str, name: &str) -> AppResult<()> {
    if name.trim().is_empty() {
        return Err(AppError::Validation(
            "Artist name cannot be empty".to_string(),
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

    let image_path = format!("/images/{name}");

    let rows_updated = clorinde::queries::artists::update_artist()
        .bind(&client, &name, &image_path, &id)
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to update artist",
                file!(),
                function_name!(),
            ))
        })?;

    if rows_updated == 0 {
        return Err(AppError::NotFound(format!(
            "Artist with id '{}' not found",
            id
        )));
    }

    Ok(())
}

#[named]
pub async fn delete_artist(pool: &Pool, id: &str) -> AppResult<()> {
    let client = pool.get().await.map_err(|err| {
        AppError::Internal(log_and_context_error(
            err,
            "Failed to get DB client",
            file!(),
            function_name!(),
        ))
    })?;

    let rows_deleted = clorinde::queries::artists::delete_artist()
        .bind(&client, &id)
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to delete artist",
                file!(),
                function_name!(),
            ))
        })?;

    if rows_deleted == 0 {
        return Err(AppError::NotFound(format!(
            "Artist with id '{}' not found",
            id
        )));
    }

    Ok(())
}
