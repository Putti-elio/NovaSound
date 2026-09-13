use connectrpc::RequestContext;
use deadpool_postgres::Pool;

use crate::adapters::connect::artist_to_proto;
use crate::errors::connect_error::to_connect_error;
use crate::rpc::novasound::artist::v1::{
    ArtistService, CreateArtistRequest, CreateArtistResponse, DeleteArtistRequest,
    DeleteArtistResponse, GetArtistRequest, GetArtistsRequest, GetArtistsResponse,
    UpdateArtistRequest, UpdateArtistResponse,
};
use novasound_application::artist_service;

#[derive(Clone)]
pub struct ConnectArtistService {
    pool: Pool,
}

impl ConnectArtistService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[allow(refining_impl_trait)]
impl ArtistService for ConnectArtistService {
    async fn get_artist(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, GetArtistRequest>,
    ) -> Result<
        connectrpc::Response<crate::rpc::novasound::artist::v1::Artist>,
        connectrpc::ConnectError,
    > {
        let artist = artist_service::get_artist(&self.pool, request.id)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(artist_to_proto(artist)))
    }

    async fn get_artists(
        &self,
        _ctx: RequestContext,
        _request: connectrpc::ServiceRequest<'_, GetArtistsRequest>,
    ) -> Result<connectrpc::Response<GetArtistsResponse>, connectrpc::ConnectError> {
        let artists = artist_service::get_all_artists(&self.pool)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(GetArtistsResponse {
            artists: artists.into_iter().map(artist_to_proto).collect(),
            ..Default::default()
        }))
    }

    async fn create_artist(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, CreateArtistRequest>,
    ) -> Result<connectrpc::Response<CreateArtistResponse>, connectrpc::ConnectError> {
        let artist = artist_service::create_artist(&self.pool, request.name)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(CreateArtistResponse {
            artist: ::buffa::MessageField::some(artist_to_proto(artist)),
            ..Default::default()
        }))
    }

    async fn update_artist(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, UpdateArtistRequest>,
    ) -> Result<connectrpc::Response<UpdateArtistResponse>, connectrpc::ConnectError> {
        let artist = artist_service::update_artist(&self.pool, request.id, request.name)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(UpdateArtistResponse {
            artist: ::buffa::MessageField::some(artist_to_proto(artist)),
            ..Default::default()
        }))
    }

    async fn delete_artist(
        &self,
        _ctx: RequestContext,
        request: connectrpc::ServiceRequest<'_, DeleteArtistRequest>,
    ) -> Result<connectrpc::Response<DeleteArtistResponse>, connectrpc::ConnectError> {
        artist_service::delete_artist(&self.pool, request.id)
            .await
            .map_err(to_connect_error)?;

        Ok(connectrpc::Response::new(DeleteArtistResponse::default()))
    }
}
