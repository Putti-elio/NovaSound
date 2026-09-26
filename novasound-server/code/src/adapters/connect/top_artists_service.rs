//! Internal transport binding only. Deliberately absent from `create_connect_router`.
use buffa::Enumeration;
use chrono::DateTime;
use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, error::ErrorDetail};
use novasound_application::top_artists::{HistoryFormat, TopArtistsService, TopArtistsStorage};
use novasound_domain::top_artists::{
    DateRange, NativePeriod, Provider, Source, TopArtists, TopArtistsError, TopArtistsProvider,
    UserId,
};

use crate::rpc::novasound::top_artists::v1::{self as proto, UserTopArtistsService};
use proto::__buffa::view::oneof::get_top_artists_request::Period;

pub struct ConnectTopArtistsService<P, S> {
    service: TopArtistsService<S>,
    user_id: UserId,
    provider: Option<P>,
}

impl<P, S: TopArtistsStorage> ConnectTopArtistsService<P, S> {
    /// Controlled tests/internal execution only. This is NOT authentication.
    /// Do not register this binding on a user-facing server. The auth task must replace
    /// this fixed identity with verified per-request context before production routing.
    pub fn for_internal_user(
        service: TopArtistsService<S>,
        user_id: UserId,
        provider: Option<P>,
    ) -> Self {
        Self {
            service,
            user_id,
            provider,
        }
    }
}

#[allow(refining_impl_trait)]
impl<P: TopArtistsProvider + 'static, S: TopArtistsStorage + 'static> UserTopArtistsService
    for ConnectTopArtistsService<P, S>
{
    async fn get_top_artists(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, proto::GetTopArtistsRequest>,
    ) -> Result<Response<proto::GetTopArtistsResponse>, ConnectError> {
        let provider = parse_provider(request.provider)?;
        let limit = if request.limit == 0 {
            50
        } else {
            request.limit
        };
        let result = match &request.period {
            | Some(Period::NativePeriod(period)) => {
                let period = match proto::NativePeriod::from_i32(period.to_i32()) {
                    | Some(proto::NativePeriod::NATIVE_PERIOD_SHORT_TERM) => {
                        NativePeriod::ShortTerm
                    },
                    | Some(proto::NativePeriod::NATIVE_PERIOD_MEDIUM_TERM) => {
                        NativePeriod::MediumTerm
                    },
                    | Some(proto::NativePeriod::NATIVE_PERIOD_LONG_TERM) => NativePeriod::LongTerm,
                    | _ => {
                        return Err(map_error(TopArtistsError::InvalidInput(
                            "invalid native period",
                        )));
                    },
                };
                let adapter = self
                    .provider
                    .as_ref()
                    .ok_or_else(|| map_error(TopArtistsError::ConnectionRequired))?;
                self.service
                    .native(self.user_id, adapter, period, limit)
                    .await
            },
            | Some(Period::CustomPeriod(period)) => {
                let start = DateTime::parse_from_rfc3339(period.start).map_err(|_| {
                    map_error(TopArtistsError::InvalidInput("invalid start timestamp"))
                })?;
                let end = DateTime::parse_from_rfc3339(period.end).map_err(|_| {
                    map_error(TopArtistsError::InvalidInput("invalid end timestamp"))
                })?;
                let range = DateRange::new(start.to_utc(), end.to_utc()).map_err(map_error)?;
                self.service
                    .custom(self.user_id, provider, range, limit)
                    .await
            },
            | None => return Err(map_error(TopArtistsError::InvalidInput("period required"))),
        }
        .map_err(map_error)?;
        Ok(Response::new(to_proto(result)))
    }

    async fn import_listening_history(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, proto::ImportListeningHistoryRequest>,
    ) -> Result<Response<proto::ImportListeningHistoryResponse>, ConnectError> {
        parse_provider(request.provider)?;
        let format = match proto::HistoryFormat::from_i32(request.format.to_i32()) {
            | Some(proto::HistoryFormat::HISTORY_FORMAT_JSON) => HistoryFormat::Json,
            | Some(proto::HistoryFormat::HISTORY_FORMAT_ZIP) => HistoryFormat::Zip,
            | _ => {
                return Err(map_error(TopArtistsError::InvalidInput(
                    "invalid history format",
                )));
            },
        };
        let summary = self
            .service
            .import_spotify_history(self.user_id, request.content.to_vec(), format)
            .await
            .map_err(map_error)?;
        Ok(Response::new(proto::ImportListeningHistoryResponse {
            inserted: summary.inserted,
            duplicates: summary.duplicates,
            skipped: summary.skipped,
            ..Default::default()
        }))
    }
}

fn parse_provider(provider: buffa::EnumValue<proto::Provider>) -> Result<Provider, ConnectError> {
    if provider == proto::Provider::PROVIDER_SPOTIFY {
        Ok(Provider::Spotify)
    } else {
        Err(map_error(TopArtistsError::InvalidInput(
            "unsupported provider",
        )))
    }
}

fn source_to_proto(source: Source) -> proto::RankingSource {
    match source {
        | Source::NativeAffinity => proto::RankingSource::RANKING_SOURCE_NATIVE_AFFINITY,
        | Source::ImportedHistory => proto::RankingSource::RANKING_SOURCE_IMPORTED_HISTORY,
    }
}

fn provider_to_proto(provider: Provider) -> proto::Provider {
    match provider {
        | Provider::Spotify => proto::Provider::PROVIDER_SPOTIFY,
    }
}

fn to_proto(top: TopArtists) -> proto::GetTopArtistsResponse {
    proto::GetTopArtistsResponse {
        entries: top
            .entries
            .into_iter()
            .map(|entry| proto::TopArtistEntry {
                rank: entry.rank,
                artist: buffa::MessageField::some(proto::ArtistIdentity {
                    id: entry.artist.id.to_string(),
                    name: entry.artist.name,
                    image_url: entry.artist.image_url,
                    providers: entry
                        .artist
                        .providers
                        .into_iter()
                        .map(|association| proto::ProviderAssociation {
                            provider: provider_to_proto(association.provider).into(),
                            artist_id: association.artist_id,
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                }),
                metrics: entry
                    .metrics
                    .map(|metrics| proto::ListeningMetrics {
                        play_count: metrics.play_count,
                        played_ms: metrics.played_ms,
                        streams_30s: metrics.streams_30s,
                        first_play: metrics.first_play.to_rfc3339(),
                        last_play: metrics.last_play.to_rfc3339(),
                        ..Default::default()
                    })
                    .into(),
                rank_change: entry.rank_change,
                source: source_to_proto(entry.source).into(),
                ..Default::default()
            })
            .collect(),
        provider: provider_to_proto(top.provider).into(),
        source: source_to_proto(top.source).into(),
        observed_at: top.observed_at.to_rfc3339(),
        comparison_at: top.comparison_at.map(|date| date.to_rfc3339()),
        history_start: top.history_start.map(|date| date.to_rfc3339()),
        history_end: top.history_end.map(|date| date.to_rfc3339()),
        ..Default::default()
    }
}

pub(crate) fn map_error(error: TopArtistsError) -> ConnectError {
    let (connect, code, retry_after) = match error {
        | TopArtistsError::InvalidInput(message) => (
            ConnectError::invalid_argument(message),
            "invalid_input",
            None,
        ),
        | TopArtistsError::ConnectionRequired => (
            ConnectError::failed_precondition("Provider connection required or expired"),
            "connection_required",
            None,
        ),
        | TopArtistsError::AccessDenied => (
            ConnectError::permission_denied("Provider access denied"),
            "access_denied",
            None,
        ),
        | TopArtistsError::RateLimited {
            retry_after_seconds,
        } => (
            ConnectError::resource_exhausted("Provider rate limited"),
            "rate_limited",
            retry_after_seconds,
        ),
        | TopArtistsError::ProviderUnavailable => (
            ConnectError::unavailable("Provider unavailable"),
            "provider_unavailable",
            None,
        ),
        | TopArtistsError::InvalidProviderResponse => (
            ConnectError::unavailable("Invalid provider response"),
            "invalid_provider_response",
            None,
        ),
        | TopArtistsError::ImportTooLarge => (
            ConnectError::resource_exhausted("Import exceeds supported limits"),
            "import_too_large",
            None,
        ),
        | TopArtistsError::ImportBusy => (
            ConnectError::resource_exhausted("History import capacity is busy"),
            "import_busy",
            None,
        ),
        | TopArtistsError::InvalidImport => (
            ConnectError::invalid_argument("Invalid extended streaming history"),
            "invalid_import",
            None,
        ),
        | TopArtistsError::StaleSynchronization => (
            ConnectError::aborted("A newer synchronization has already completed"),
            "stale_synchronization",
            None,
        ),
        | TopArtistsError::Storage(_) | TopArtistsError::Worker => (
            ConnectError::internal("Unable to complete Top Artists operation"),
            "internal",
            None,
        ),
    };
    connect.with_detail(ErrorDetail {
        type_url: "type.novasound.dev/top-artists-error".into(),
        value: None,
        debug: Some(serde_json::json!({"code": code, "retry_after_seconds": retry_after})),
    })
}
