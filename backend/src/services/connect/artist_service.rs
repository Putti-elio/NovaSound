use connectrpc::Context;
use deadpool_postgres::Pool;

use crate::rpc::novasound::artist::v1::{
    ArtistService, CreateArtistRequestView, CreateArtistResponse, DeleteArtistRequestView,
    DeleteArtistResponse, GetArtistRequestView, GetArtistsRequestView, GetArtistsResponse,
    UpdateArtistRequestView, UpdateArtistResponse,
};
use crate::services::artist_service;
use crate::services::connect::artist_to_proto;

#[derive(Clone)]
pub struct ConnectArtistService {
    pool: Pool,
}

impl ConnectArtistService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

impl ArtistService for ConnectArtistService {
    async fn get_artist(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<GetArtistRequestView<'static>>,
    ) -> Result<(crate::rpc::novasound::artist::v1::Artist, Context), connectrpc::ConnectError>
    {
        let artist = artist_service::get_artist(&self.pool, request.id)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((artist_to_proto(artist), ctx))
    }

    async fn get_artists(
        &self,
        ctx: Context,
        _request: ::buffa::view::OwnedView<GetArtistsRequestView<'static>>,
    ) -> Result<(GetArtistsResponse, Context), connectrpc::ConnectError> {
        let artists = artist_service::get_all_artists(&self.pool)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((
            GetArtistsResponse {
                artists: artists.into_iter().map(artist_to_proto).collect(),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn create_artist(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<CreateArtistRequestView<'static>>,
    ) -> Result<(CreateArtistResponse, Context), connectrpc::ConnectError> {
        let artist = artist_service::create_artist(&self.pool, request.name)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((
            CreateArtistResponse {
                artist: ::buffa::MessageField::some(artist_to_proto(artist)),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn update_artist(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<UpdateArtistRequestView<'static>>,
    ) -> Result<(UpdateArtistResponse, Context), connectrpc::ConnectError> {
        let artist = artist_service::update_artist(&self.pool, request.id, request.name)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((
            UpdateArtistResponse {
                artist: ::buffa::MessageField::some(artist_to_proto(artist)),
                ..Default::default()
            },
            ctx,
        ))
    }

    async fn delete_artist(
        &self,
        ctx: Context,
        request: ::buffa::view::OwnedView<DeleteArtistRequestView<'static>>,
    ) -> Result<(DeleteArtistResponse, Context), connectrpc::ConnectError> {
        artist_service::delete_artist(&self.pool, request.id)
            .await
            .map_err(connectrpc::ConnectError::from)?;

        Ok((DeleteArtistResponse::default(), ctx))
    }
}
