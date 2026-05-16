use axum::{
    Router,
    routing::{delete, get, post, put},
};
use deadpool_postgres::Pool;

pub mod album_route;
mod artist_route;
mod song_route;

pub type SharedDatabase = Pool;

pub fn create_router(database: SharedDatabase) -> Router {
    Router::new()
        .route("/artists", get(artist_route::get_all_artists))
        .route("/artists/{id}", get(artist_route::get_artist))
        .route("/artists", post(artist_route::create_artist))
        .route("/artists/{id}", put(artist_route::update_artist))
        .route("/artists/{id}", delete(artist_route::delete_artist))
        .route("/albums", get(album_route::get_all_albums))
        .route("/albums/{id}", get(album_route::get_album))
        .route("/albums", post(album_route::create_album))
        .route("/albums/{id}", put(album_route::update_album))
        .route("/albums/{id}", delete(album_route::delete_album))
        .route(
            "/artists/{id}/albums",
            get(album_route::get_albums_by_artist),
        )
        .route("/songs", get(song_route::get_all_songs))
        .route("/songs", post(song_route::create_song))
        .route("/songs/{id}", get(song_route::get_song))
        .route("/songs/{id}", put(song_route::update_song))
        .route("/songs/{id}", delete(song_route::delete_song))
        .route("/artists/{id}/songs", get(song_route::get_songs_by_artist))
        .route("/albums/{id}/songs", get(song_route::get_songs_by_album))
        .with_state(database)
}
