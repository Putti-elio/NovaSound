use axum::{
    Json, debug_handler,
    extract::{Path, State},
};

use crate::{
    errors::AppResult,
    models::album_model::{Album, CreateAlbum, UpdateAlbum},
    routes::SharedDatabase,
    services::album_service,
};

#[debug_handler]
pub async fn get_all_albums(State(database): State<SharedDatabase>) -> AppResult<Json<Vec<Album>>> {
    let albums = album_service::get_all_albums(&database).await?;
    Ok(Json(albums))
}

#[debug_handler]
pub async fn get_album(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> AppResult<Json<Album>> {
    let album = album_service::get_album_by_id(&database, &id).await?;
    Ok(Json(album))
}

#[debug_handler]
pub async fn get_albums_by_artist(
    State(database): State<SharedDatabase>,
    Path(artist_id): Path<String>,
) -> AppResult<Json<Vec<Album>>> {
    let albums = album_service::get_albums_by_artist(&database, &artist_id).await?;
    Ok(Json(albums))
}

#[debug_handler]
pub async fn create_album(
    State(database): State<SharedDatabase>,
    Json(album): Json<CreateAlbum>,
) -> AppResult<Json<serde_json::Value>> {
    let id = album_service::create_album(&database, album).await?;
    Ok(Json(serde_json::json!({
        "id": id,
        "message": "Album created successfully"
    })))
}

#[debug_handler]
pub async fn update_album(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
    Json(album): Json<UpdateAlbum>,
) -> AppResult<Json<serde_json::Value>> {
    album_service::update_album(&database, &id, album).await?;
    Ok(Json(serde_json::json!({
        "message": "Album updated successfully"
    })))
}

#[debug_handler]
pub async fn delete_album(
    State(database): State<SharedDatabase>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    album_service::delete_album(&database, &id).await?;
    Ok(Json(serde_json::json!({
        "message": "Album deleted successfully"
    })))
}
