use connectrpc::Context;
use deadpool_postgres::Pool;

use crate::models::song_model::{CreateSong, UpdateSong};
use crate::rpc::novasound::song::v1::{
    CreateSongRequestView, CreateSongResponse, DeleteSongRequestView, DeleteSongResponse,
    GetSongRequestView, GetSongsRequestView, GetSongsResponse, SongService, UpdateSongRequestView,
    UpdateSongResponse,
};
use crate::services::connect::{map_app_error, parse_optional_date, song_to_proto};
use crate::services::song_service;

#[derive(Clone)]
pub struct ConnectSongService {
    pool: Pool,
}

impl ConnectSongService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

impl SongService for ConnectSongService {
    async fn get_song(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<GetSongRequestView<'static>>,
    ) -> Result<(crate::rpc::novasound::song::v1::Song, Context), connectrpc::ConnectError> {
        let song = song_service::get_song_by_id(&self.pool, request.id)
            .await
            .map_err(map_app_error)?;

        Ok((song_to_proto(song), ctx))
    }

    async fn get_songs(
        &self,
        ctx: Context,
        _request: ::buffa::view::OwnedView<GetSongsRequestView<'static>>,
    ) -> Result<(GetSongsResponse, Context), connectrpc::ConnectError> {
        let songs = song_service::get_all_songs(&self.pool)
            .await
            .map_err(map_app_error)?;

        Ok((
            GetSongsResponse {
                songs: songs.into_iter().map(song_to_proto).collect(),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn create_song(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<CreateSongRequestView<'static>>,
    ) -> Result<(CreateSongResponse, Context), connectrpc::ConnectError> {
        let song = CreateSong {
            name: request.name.to_string(),
            duration: request.duration,
            artist_id: request.artist_id.to_string(),
            album_id: request.album_id.map(str::to_owned),
            release_date: parse_optional_date(request.release_date)?,
            track_number: request.track_number,
        };

        let created_song = song_service::create_song(&self.pool, song)
            .await
            .map_err(map_app_error)?;

        Ok((
            CreateSongResponse {
                song: ::buffa::MessageField::some(song_to_proto(created_song)),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn update_song(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<UpdateSongRequestView<'static>>,
    ) -> Result<(UpdateSongResponse, Context), connectrpc::ConnectError> {
        let song = UpdateSong {
            name: request.name.map(str::to_owned),
            duration: request.duration,
            release_date: parse_optional_date(request.release_date)?,
            track_number: request.track_number,
        };

        let updated_song = song_service::update_song(&self.pool, request.id, song)
            .await
            .map_err(map_app_error)?;

        Ok((
            UpdateSongResponse {
                song: ::buffa::MessageField::some(song_to_proto(updated_song)),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn delete_song(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<DeleteSongRequestView<'static>>,
    ) -> Result<(DeleteSongResponse, Context), connectrpc::ConnectError> {
        song_service::delete_song(&self.pool, request.id)
            .await
            .map_err(map_app_error)?;

        Ok((DeleteSongResponse::default(), ctx))
    }
}
