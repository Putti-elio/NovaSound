use connectrpc::RequestContext;
use deadpool_postgres::Pool;

use crate::adapters::connect::{parse_optional_date, song_to_proto};
use crate::errors::connect_error::{to_connect_error, validation_error};
use crate::models::song_model::{CreateSong, UpdateSong};
use crate::rpc::novasound::song::v1::{
    CreateSongRequest, CreateSongResponse, DeleteSongRequest, DeleteSongResponse, GetSongRequest,
    GetSongsRequest, GetSongsResponse, SongService, UpdateSongRequest, UpdateSongResponse,
};
use novasound_application::song_service;
use novasound_domain::validation::ValidationErrors;

#[derive(Clone)]
pub struct ConnectSongService {
    pool: Pool,
}

impl ConnectSongService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[allow(refining_impl_trait)]
impl SongService for ConnectSongService {
    async fn get_song(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, GetSongRequest>,
    ) -> Result<connectrpc::Response<crate::rpc::novasound::song::v1::Song>, connectrpc::ConnectError>
    {
        let song = song_service::get_song_by_id(&self.pool, request.id)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(song_to_proto(song)))
    }

    async fn get_songs(
        &self,
        _ctx: RequestContext,
        _request: connectrpc::ServiceRequest<'_, GetSongsRequest>,
    ) -> Result<connectrpc::Response<GetSongsResponse>, connectrpc::ConnectError> {
        let songs = song_service::get_all_songs(&self.pool)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(GetSongsResponse {
            songs: songs.into_iter().map(song_to_proto).collect(),
            ..Default::default()
        }))
    }

    async fn create_song(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, CreateSongRequest>,
    ) -> Result<connectrpc::Response<CreateSongResponse>, connectrpc::ConnectError> {
        let release_date = parse_optional_date(request.release_date);
        let song = CreateSong {
            name: request.name.to_string(),
            duration: request.duration,
            artist_id: request.artist_id.to_string(),
            album_id: request.album_id.map(str::to_owned),
            release_date: release_date.clone().unwrap_or(None),
            track_number: request.track_number,
        };
        let mut errors = ValidationErrors::new();
        if let Err(error) = release_date {
            errors.push(error);
        }
        if let Err(error) = song.validate() {
            errors.extend(error);
        }
        if !errors.is_empty() {
            return Err(validation_error(errors));
        }

        let created_song = song_service::create_song(&self.pool, song)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(CreateSongResponse {
            song: ::buffa::MessageField::some(song_to_proto(created_song)),
            ..Default::default()
        }))
    }

    async fn update_song(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, UpdateSongRequest>,
    ) -> Result<connectrpc::Response<UpdateSongResponse>, connectrpc::ConnectError> {
        let release_date = parse_optional_date(request.release_date);
        let song = UpdateSong {
            name: request.name.map(str::to_owned),
            duration: request.duration,
            release_date: release_date.clone().unwrap_or(None),
            track_number: request.track_number,
        };
        let mut errors = ValidationErrors::new();
        if let Err(error) = release_date {
            errors.push(error);
        }
        if let Err(error) = song.validate() {
            errors.extend(error);
        }
        if !errors.is_empty() {
            return Err(validation_error(errors));
        }

        let updated_song = song_service::update_song(&self.pool, request.id, song)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(UpdateSongResponse {
            song: ::buffa::MessageField::some(song_to_proto(updated_song)),
            ..Default::default()
        }))
    }

    async fn delete_song(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, DeleteSongRequest>,
    ) -> Result<connectrpc::Response<DeleteSongResponse>, connectrpc::ConnectError> {
        song_service::delete_song(&self.pool, request.id)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(DeleteSongResponse::default()))
    }
}
