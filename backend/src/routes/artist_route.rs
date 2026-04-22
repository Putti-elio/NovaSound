#![allow(clippy::unwrap_used)]

use anyhow::Result;
use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::StatusCode,
};
use function_name::named;
use log::{error, info};
use uuid::Uuid;

use crate::models::artist_model::{Artist, CreateArtist, UpdateArtist};
use crate::routes::SharedDatabase;
use crate::services::artist_service;

// GET
#[named]
pub async fn get_all_artists(
    State(database): State<SharedDatabase>,
) -> Result<Json<Vec<Artist>>, StatusCode> {
    let db = database.lock().unwrap();

    match artist_service::get_all_artists(&db) {
        | Ok(artists) => Ok(Json(artists)),
        | Err(err) => {
            error!(
                "Database error couldn't get artists. {} At {}::{} ",
                err,
                file!(),
                function_name!()
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

// POST
#[named]
pub async fn create_artist(
    State(database): State<SharedDatabase>,
    Json(payload): Json<CreateArtist>,
) -> (StatusCode, &'static str) {
    let db = database.lock().unwrap();

    info!("{}", &payload.name);

    match artist_service::create_artist(&db, &payload.name) {
        | Ok(()) => (StatusCode::CREATED, "Artist created successfully"),
        | Err(err) => {
            error!(
                "Database error couldn't create artist. {} At {}::{} ",
                err,
                file!(),
                function_name!()
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create artist")
        },
    }
}

#[named]
#[debug_handler]
pub async fn get_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<Uuid>,
) -> Result<Json<Artist>, StatusCode> {
    let db = database.lock().unwrap();

    match artist_service::get_artist(&db, &id.to_string()) {
        | Ok(artist) => Ok(Json(artist)),
        | Err(err) => {
            error!(
                "Database error couldn't get artist. {} At {}::{} ",
                err,
                file!(),
                function_name!()
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

#[named]
#[debug_handler]
pub async fn update_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateArtist>,
) -> (StatusCode, &'static str) {
    let db = database.lock().unwrap();

    match artist_service::update_artist(&db, &id.to_string(), &payload.name) {
        | Ok(()) => (StatusCode::OK, "Artist updated successfully"),
        | Err(err) => {
            error!(
                "Database error couldn't update artist. {} At {}::{} ",
                err,
                file!(),
                function_name!()
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update artist")
        },
    }
}

#[named]
#[debug_handler]
pub async fn delete_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<Uuid>,
) -> (StatusCode, &'static str) {
    let db = database.lock().unwrap();

    match artist_service::delete_artist(&db, &id.to_string()) {
        | Ok(()) => (StatusCode::OK, "Artist deleted successfully"),
        | Err(err) => {
            error!(
                "Database error couldn't delete artist. {} At {}::{} ",
                err,
                file!(),
                function_name!()
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete artist")
        },
    }
}
