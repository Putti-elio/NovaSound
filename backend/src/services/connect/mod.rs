use std::sync::Arc;

use chrono::NaiveDate;
use connectrpc::Router;
use deadpool_postgres::Pool;

use crate::errors::AppError;
use crate::models::date_serde::DATE_FORMAT;
use crate::rpc::novasound::album::v1::AlbumServiceExt;
use crate::rpc::novasound::artist::v1::ArtistServiceExt;
use crate::rpc::novasound::song::v1::SongServiceExt;
use crate::rpc::novasound::{album::v1, artist::v1 as artist_v1, song::v1 as song_v1};

pub mod album_service;
pub mod artist_service;
pub mod song_service;

pub fn create_connect_router(pool: Pool) -> Router {
    let router = Router::new();
    let router = Arc::new(artist_service::ConnectArtistService::new(pool.clone())).register(router);
    let router = Arc::new(album_service::ConnectAlbumService::new(pool.clone())).register(router);

    Arc::new(song_service::ConnectSongService::new(pool)).register(router)
}

fn map_app_error(error: AppError) -> connectrpc::ConnectError {
    match error {
        | AppError::NotFound(message) => connectrpc::ConnectError::not_found(message),
        | AppError::Validation(message) => connectrpc::ConnectError::invalid_argument(message),
        | AppError::Internal(error) => connectrpc::ConnectError::internal(error.to_string()),
    }
}

#[allow(clippy::result_large_err)]
fn parse_optional_date(value: Option<&str>) -> Result<Option<NaiveDate>, connectrpc::ConnectError> {
    value
        .map(|date| {
            NaiveDate::parse_from_str(date, DATE_FORMAT).map_err(|_| {
                connectrpc::ConnectError::invalid_argument(
                    "Invalid date format. Expected format is DD-MM-YYYY",
                )
            })
        })
        .transpose()
}

fn format_optional_date(value: Option<NaiveDate>) -> Option<String> {
    value.map(|date| date.format(DATE_FORMAT).to_string())
}

fn artist_to_proto(artist: crate::models::artist_model::Artist) -> artist_v1::Artist {
    artist_v1::Artist {
        id: artist.id,
        name: artist.name,
        image_path: artist.image_path,
        ..Default::default()
    }
}

fn album_type_to_proto(album_type: crate::models::song_model::AlbumType) -> v1::AlbumType {
    match album_type {
        | crate::models::song_model::AlbumType::Album => v1::AlbumType::ALBUM_TYPE_ALBUM,
        | crate::models::song_model::AlbumType::Ep => v1::AlbumType::ALBUM_TYPE_EP,
        | crate::models::song_model::AlbumType::Single => v1::AlbumType::ALBUM_TYPE_SINGLE,
        | crate::models::song_model::AlbumType::StandaloneCollection => {
            v1::AlbumType::ALBUM_TYPE_STANDALONE_COLLECTION
        },
    }
}

#[allow(clippy::result_large_err)]
fn proto_album_type_to_model(
    album_type: Option<::buffa::EnumValue<v1::AlbumType>>,
) -> Result<Option<crate::models::song_model::AlbumType>, connectrpc::ConnectError> {
    use buffa::Enumeration;

    match album_type {
        | None => Ok(None),
        | Some(value) => match v1::AlbumType::from_i32(value.to_i32()) {
            | Some(v1::AlbumType::ALBUM_TYPE_UNSPECIFIED) => Ok(None),
            | Some(v1::AlbumType::ALBUM_TYPE_ALBUM) => {
                Ok(Some(crate::models::song_model::AlbumType::Album))
            },
            | Some(v1::AlbumType::ALBUM_TYPE_EP) => {
                Ok(Some(crate::models::song_model::AlbumType::Ep))
            },
            | Some(v1::AlbumType::ALBUM_TYPE_SINGLE) => {
                Ok(Some(crate::models::song_model::AlbumType::Single))
            },
            | Some(v1::AlbumType::ALBUM_TYPE_STANDALONE_COLLECTION) => Ok(Some(
                crate::models::song_model::AlbumType::StandaloneCollection,
            )),
            | None => Err(connectrpc::ConnectError::invalid_argument(
                "Invalid album type value",
            )),
        },
    }
}

fn album_to_proto(album: crate::models::album_model::Album) -> v1::Album {
    v1::Album {
        id: album.id,
        name: album.name,
        total_duration: album.total_duration,
        release_date: format_optional_date(album.release_date),
        artist_id: album.artist_id,
        image_path: album.image_path,
        album_type: album_type_to_proto(album.album_type).into(),
        ..Default::default()
    }
}

fn song_to_proto(song: crate::models::song_model::Song) -> song_v1::Song {
    song_v1::Song {
        id: song.id,
        name: song.name,
        duration: song.duration,
        artist_id: song.artist_id,
        album_id: song.album_id,
        release_date: format_optional_date(song.release_date),
        track_number: song.track_number,
        image_path: song.image_path,
        ..Default::default()
    }
}
