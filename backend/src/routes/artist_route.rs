use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};
use log::info;
use uuid::Uuid;

use crate::errors::AppResult;
use crate::models::artist_model::{Artist, CreateArtist, UpdateArtist};
use crate::routes::SharedDatabase;
use crate::services::artist_service;

#[debug_handler]
pub async fn get_all_artists(
    State(database): State<SharedDatabase>,
) -> AppResult<Json<Vec<Artist>>> {
    let artists = artist_service::get_all_artists(&database).await?;
    Ok(Json(artists))
}

#[debug_handler]
pub async fn create_artist(
    State(database): State<SharedDatabase>,
    Json(payload): Json<CreateArtist>,
) -> AppResult<(StatusCode, &'static str)> {
    info!("{}", &payload.name);
    artist_service::create_artist(&database, &payload.name).await?;
    Ok((StatusCode::CREATED, "Artist created successfully"))
}

#[debug_handler]
pub async fn get_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Artist>> {
    let artist = artist_service::get_artist(&database, &id.to_string()).await?;
    Ok(Json(artist))
}

#[debug_handler]
pub async fn update_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateArtist>,
) -> AppResult<(StatusCode, &'static str)> {
    artist_service::update_artist(&database, &id.to_string(), &payload.name).await?;
    Ok((StatusCode::OK, "Artist updated successfully"))
}

#[debug_handler]
pub async fn delete_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<Uuid>,
) -> AppResult<(StatusCode, &'static str)> {
    artist_service::delete_artist(&database, &id.to_string()).await?;
    Ok((StatusCode::OK, "Artist deleted successfully"))
}
