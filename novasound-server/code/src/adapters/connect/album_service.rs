use crate::adapters::connect::{album_to_proto, parse_optional_date, proto_album_type_to_model};
use crate::errors::connect_error::{to_connect_error, validation_error};
use crate::models::album_model::{CreateAlbum, UpdateAlbum};
use crate::rpc::novasound::album::v1::{
    AlbumService, CreateAlbumRequest, CreateAlbumResponse, DeleteAlbumRequest, DeleteAlbumResponse,
    GetAlbumRequest, GetAlbumsRequest, GetAlbumsResponse, UpdateAlbumRequest, UpdateAlbumResponse,
};
use connectrpc::RequestContext;
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

#[allow(refining_impl_trait)]
impl AlbumService for ConnectAlbumService {
    async fn get_album(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, GetAlbumRequest>,
    ) -> Result<
        connectrpc::Response<crate::rpc::novasound::album::v1::Album>,
        connectrpc::ConnectError,
    > {
        let album = album_service::get_album_by_id(&self.pool, request.id)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(album_to_proto(album)))
    }

    async fn get_albums(
        &self,
        _ctx: RequestContext,
        _request: connectrpc::ServiceRequest<'_, GetAlbumsRequest>,
    ) -> Result<connectrpc::Response<GetAlbumsResponse>, connectrpc::ConnectError> {
        let albums = album_service::get_all_albums(&self.pool)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(GetAlbumsResponse {
            albums: albums.into_iter().map(album_to_proto).collect(),
            ..Default::default()
        }))
    }

    async fn create_album(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, CreateAlbumRequest>,
    ) -> Result<connectrpc::Response<CreateAlbumResponse>, connectrpc::ConnectError> {
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

        Ok(connectrpc::Response::new(CreateAlbumResponse {
            album: ::buffa::MessageField::some(album_to_proto(created_album)),
            ..Default::default()
        }))
    }

    async fn update_album(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, UpdateAlbumRequest>,
    ) -> Result<connectrpc::Response<UpdateAlbumResponse>, connectrpc::ConnectError> {
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

        Ok(connectrpc::Response::new(UpdateAlbumResponse {
            album: ::buffa::MessageField::some(album_to_proto(updated_album)),
            ..Default::default()
        }))
    }

    async fn delete_album(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, DeleteAlbumRequest>,
    ) -> Result<connectrpc::Response<DeleteAlbumResponse>, connectrpc::ConnectError> {
        album_service::delete_album(&self.pool, request.id)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(DeleteAlbumResponse::default()))
    }
}
