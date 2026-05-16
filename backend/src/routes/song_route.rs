use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};

use crate::errors::AppResult;
use crate::models::song_model::{CreateSong, Song, UpdateSong};
use crate::routes::SharedDatabase;
use crate::services::song_service;

#[debug_handler]
pub async fn get_all_songs(State(database): State<SharedDatabase>) -> AppResult<Json<Vec<Song>>> {
    let songs = song_service::get_all_songs(&database).await?;
    Ok(Json(songs))
}

#[debug_handler]
pub async fn get_song(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> AppResult<Json<Song>> {
    let song = song_service::get_song_by_id(&database, &id).await?;
    Ok(Json(song))
}

#[debug_handler]
pub async fn get_songs_by_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> AppResult<Json<Vec<Song>>> {
    let songs = song_service::get_songs_by_artist(&database, &id).await?;
    Ok(Json(songs))
}

#[debug_handler]
pub async fn get_songs_by_album(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> AppResult<Json<Vec<Song>>> {
    let songs = song_service::get_songs_by_album(&database, &id).await?;
    Ok(Json(songs))
}

#[debug_handler]
pub async fn create_song(
    State(database): State<SharedDatabase>,
    Json(song): Json<CreateSong>,
) -> AppResult<(StatusCode, Json<String>)> {
    let id = song_service::create_song(&database, song).await?;
    Ok((StatusCode::CREATED, Json(id)))
}

#[debug_handler]
pub async fn update_song(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
    Json(song): Json<UpdateSong>,
) -> AppResult<StatusCode> {
    song_service::update_song(&database, &id, song).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn delete_song(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    song_service::delete_song(&database, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
