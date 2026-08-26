use crate::models::album_model::{CreateAlbum, UpdateAlbum};
use crate::rpc::novasound::album::v1::{
    AlbumService, CreateAlbumRequestView, CreateAlbumResponse, DeleteAlbumRequestView,
    DeleteAlbumResponse, GetAlbumRequestView, GetAlbumsRequestView, GetAlbumsResponse,
    UpdateAlbumRequestView, UpdateAlbumResponse,
};
use crate::services::album_service;
use crate::services::connect::{album_to_proto, parse_optional_date, proto_album_type_to_model};
use connectrpc::Context;
use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct ConnectAlbumService {
    pool: Pool,
}

impl ConnectAlbumService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

impl AlbumService for ConnectAlbumService {
    async fn get_album(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<GetAlbumRequestView<'static>>,
    ) -> Result<(crate::rpc::novasound::album::v1::Album, Context), connectrpc::ConnectError> {
        let album = album_service::get_album_by_id(&self.pool, request.id)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((album_to_proto(album), ctx))
    }

    async fn get_albums(
        &self,
        ctx: Context,
        _request: ::buffa::view::OwnedView<GetAlbumsRequestView<'static>>,
    ) -> Result<(GetAlbumsResponse, Context), connectrpc::ConnectError> {
        let albums = album_service::get_all_albums(&self.pool)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((
            GetAlbumsResponse {
                albums: albums.into_iter().map(album_to_proto).collect(),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn create_album(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<CreateAlbumRequestView<'static>>,
    ) -> Result<(CreateAlbumResponse, Context), connectrpc::ConnectError> {
        let album = CreateAlbum {
            name: request.name.to_string(),
            release_date: parse_optional_date(request.release_date)?,
            artist_id: request.artist_id.to_string(),
            album_type: proto_album_type_to_model(request.album_type)?,
        };

        let created_album = album_service::create_album(&self.pool, album)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((
            CreateAlbumResponse {
                album: ::buffa::MessageField::some(album_to_proto(created_album)),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn update_album(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<UpdateAlbumRequestView<'static>>,
    ) -> Result<(UpdateAlbumResponse, Context), connectrpc::ConnectError> {
        let album = UpdateAlbum {
            name: request.name.map(str::to_owned),
            release_date: parse_optional_date(request.release_date)?,
            artist_id: request.artist_id.map(str::to_owned),
        };

        let updated_album = album_service::update_album(&self.pool, request.id, album)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((
            UpdateAlbumResponse {
                album: ::buffa::MessageField::some(album_to_proto(updated_album)),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn delete_album(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<DeleteAlbumRequestView<'static>>,
    ) -> Result<(DeleteAlbumResponse, Context), connectrpc::ConnectError> {
        album_service::delete_album(&self.pool, request.id)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((DeleteAlbumResponse::default(), ctx))
    }
}
