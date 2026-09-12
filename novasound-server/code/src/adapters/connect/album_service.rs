use crate::adapters::connect::{album_to_proto, parse_optional_date, proto_album_type_to_model};
use crate::errors::connect_error::{to_connect_error, validation_error};
use crate::models::album_model::{CreateAlbum, UpdateAlbum};
use crate::rpc::novasound::album::v1::{
    AlbumService, CreateAlbumRequestView, CreateAlbumResponse, DeleteAlbumRequestView,
    DeleteAlbumResponse, GetAlbumRequestView, GetAlbumsRequestView, GetAlbumsResponse,
    UpdateAlbumRequestView, UpdateAlbumResponse,
};
use connectrpc::Context;
use deadpool_postgres::Pool;
use novasound_application::album_service;
use novasound_domain::validation::ValidationErrors;

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
            .map_err(to_connect_error)?;

        Ok((album_to_proto(album), ctx))
    }

    async fn get_albums(
        &self,
        ctx: Context,
        _request: ::buffa::view::OwnedView<GetAlbumsRequestView<'static>>,
    ) -> Result<(GetAlbumsResponse, Context), connectrpc::ConnectError> {
        let albums = album_service::get_all_albums(&self.pool)
            .await
            .map_err(to_connect_error)?;

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
        let release_date = parse_optional_date(request.release_date);
        let album_type = proto_album_type_to_model(request.album_type);
        let album = CreateAlbum {
            name: request.name.to_string(),
            release_date: release_date.clone().unwrap_or(None),
            artist_id: request.artist_id.to_string(),
            album_type: album_type.clone().unwrap_or(None),
        };
        let mut errors = ValidationErrors::new();
        if let Err(error) = release_date {
            errors.push(error);
        }
        if let Err(error) = album_type {
            errors.push(error);
        }
        if let Err(error) = album.validate() {
            errors.extend(error);
        }
        if !errors.is_empty() {
            return Err(validation_error(errors));
        }

        let created_album = album_service::create_album(&self.pool, album)
            .await
            .map_err(to_connect_error)?;

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
        let release_date = parse_optional_date(request.release_date);
        let album = UpdateAlbum {
            name: request.name.map(str::to_owned),
            release_date: release_date.clone().unwrap_or(None),
            artist_id: request.artist_id.map(str::to_owned),
        };
        let mut errors = ValidationErrors::new();
        if let Err(error) = release_date {
            errors.push(error);
        }
        if let Err(error) = album.validate() {
            errors.extend(error);
        }
        if !errors.is_empty() {
            return Err(validation_error(errors));
        }

        let updated_album = album_service::update_album(&self.pool, request.id, album)
            .await
            .map_err(to_connect_error)?;

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
            .map_err(to_connect_error)?;

        Ok((DeleteAlbumResponse::default(), ctx))
    }
}
