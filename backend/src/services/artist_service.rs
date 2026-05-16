use deadpool_postgres::Pool;
use function_name::named;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::artist_model::Artist;
use crate::utils::log_and_context_error;

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

    let rows = client
        .query("SELECT id, name, image_path FROM artists", &[])
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to execute the artist query",
                file!(),
                function_name!(),
            ))
        })?;

    let artists = rows
        .iter()
        .map(|row| Artist {
            id: row.get(0),
            name: row.get(1),
            image_path: row.get(2),
        })
        .collect();

    Ok(artists)
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

    let exists: bool = client
        .query_one("SELECT 1 FROM artists WHERE name = $1", &[&name])
        .await
        .is_ok();

    if exists {
        return Err(AppError::Validation(format!(
            "Artist '{}' already exists",
            name
        )));
    }

    let id = Uuid::new_v4().to_string();
    let image_path = "/images/".to_owned() + name;

    client
        .execute(
            "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
            &[&id, &name, &image_path],
        )
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

    let row = client
        .query_opt(
            "SELECT id, name, image_path FROM artists WHERE id = $1",
            &[id],
        )
        .await
        .map_err(|err| {
            AppError::Internal(log_and_context_error(
                err,
                "Failed to execute the artist query",
                file!(),
                function_name!(),
            ))
        })?
        .ok_or_else(|| AppError::NotFound(format!("Artist with id '{}' not found", id)))?;

    Ok(Artist {
        id: row.get(0),
        name: row.get(1),
        image_path: row.get(2),
    })
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

    let rows_updated = client
        .execute(
            "UPDATE artists SET name = $1, image_path = $2 WHERE id = $3",
            &[&name, &format!("/images/{}", name), &id],
        )
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

    let rows_deleted = client
        .execute("DELETE FROM artists WHERE id = $1", &[&id])
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
