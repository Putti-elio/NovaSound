use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::models::song_model::{CreateSong, Song, UpdateSong};
use crate::routes::SharedDatabase;
use crate::services::song_service;

#[debug_handler]
pub async fn get_all_songs(
    State(database): State<SharedDatabase>,
) -> Result<Json<Vec<Song>>, StatusCode> {
    let conn = database.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let songs = song_service::get_all_songs(&conn)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(songs))
}

#[debug_handler]
pub async fn get_song(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> Result<Json<Song>, StatusCode> {
    let conn = database.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let song = song_service::get_song_by_id(&conn, &id)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(song))
}

#[debug_handler]
pub async fn get_songs_by_artist(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Song>>, StatusCode> {
    let conn = database.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let songs = song_service::get_songs_by_artist(&conn, &id)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(songs))
}

#[debug_handler]
pub async fn get_songs_by_album(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Song>>, StatusCode> {
    let conn = database.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let songs = song_service::get_songs_by_album(&conn, &id)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(songs))
}

#[debug_handler]
pub async fn create_song(
    State(database): State<SharedDatabase>,
    Json(song): Json<CreateSong>,
) -> Result<(StatusCode, Json<String>), StatusCode> {
    let conn = database.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let id = song_service::create_song(&conn, song)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(id)))
}

#[debug_handler]
pub async fn update_song(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
    Json(song): Json<UpdateSong>,
) -> Result<StatusCode, StatusCode> {
    let conn = database.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    song_service::update_song(&conn, &id, song)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn delete_song(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let conn = database.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    song_service::delete_song(&conn, &id)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
