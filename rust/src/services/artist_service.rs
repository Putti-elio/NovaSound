use anyhow::{Context, Result};
use function_name::named;
use rusqlite::{params, Connection, Error as RusqliteError};
use uuid::Uuid;

use crate::create_error;
use crate::models::artist_model::Artist;
use crate::utils::log_and_context_error;

#[named]
pub fn get_all_artists(database: &Connection) -> Result<Vec<Artist>> {
    let mut statement = database
        .prepare("SELECT id, name, image_path FROM artists")
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare the artist query",
                file!(),
                function_name!(),
            )
        })?;

    let result_statement = statement
        .query_map([], |row| {
            Ok(Artist {
                id: row.get(0)?,
                name: row.get(1)?,
                image_path: row.get(2)?,
            })
        })
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to execute the artist query",
                file!(),
                function_name!(),
            )
        })?;

    let artists = result_statement
        .collect::<Result<Vec<Artist>, RusqliteError>>()
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to collect artists from iterator",
                file!(),
                function_name!(),
            )
        })?;

    Ok(artists)
}

#[named]
pub fn create_artist(database: &Connection, name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return create_error!("Artist name cannot be empty.");
    }

    let mut stmt = database
        .prepare("SELECT 1 FROM artists WHERE name = ?1")
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare statement to check for existing artist.",
                file!(),
                function_name!(),
            )
        })?;

    let exists: bool = stmt.exists(params![name]).map_err(|err| {
        log_and_context_error(
            err,
            "Failed to check if artist exists.",
            file!(),
            function_name!(),
        )
    })?;

    if exists {
        return create_error!("Artist '{}' already exists.", name);
    }

    let id = Uuid::new_v4().to_string();
    let image_path = "/images/".to_owned() + name;

    database
        .execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![id, name, image_path],
        )
        .with_context(|| format!("Failed to insert artist '{}'", name))?;

    Ok(())
}

#[named]
pub fn get_artist(database: &Connection, id: &String) -> Result<Artist> {
    let mut statement = database
        .prepare("SELECT id, name, image_path FROM artists WHERE id = ?1")
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to prepare the artist query",
                file!(),
                function_name!(),
            )
        })?;

    let artist = statement
        .query_row(params![id], |row| {
            Ok(Artist {
                id: row.get(0)?,
                name: row.get(1)?,
                image_path: row.get(2)?,
            })
        })
        .map_err(|err| {
            log_and_context_error(
                err,
                "Failed to execute the artist query",
                file!(),
                function_name!(),
            )
        })?;

    Ok(artist)
}

#[named]
pub fn update_artist(database: &Connection, id: &str, name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return create_error!("Artist name cannot be empty.");
    }

    let rows_updated = database
        .execute(
            "UPDATE artists SET name = ?1, image_path = ?2 WHERE id = ?3",
            params![name, format!("/images/{}", name), id],
        )
        .map_err(|err| {
            log_and_context_error(err, "Failed to update artist", file!(), function_name!())
        })?;

    if rows_updated == 0 {
        return create_error!("Artist with id '{}' not found.", id);
    }

    Ok(())
}

#[named]
pub fn delete_artist(database: &Connection, id: &str) -> Result<()> {
    let rows_deleted = database
        .execute("DELETE FROM artists WHERE id = ?1", params![id])
        .map_err(|err| {
            log_and_context_error(err, "Failed to delete artist", file!(), function_name!())
        })?;

    if rows_deleted == 0 {
        return create_error!("Artist with id '{}' not found.", id);
    }

    Ok(())
}
