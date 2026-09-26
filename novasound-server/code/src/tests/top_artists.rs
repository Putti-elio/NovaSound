#![allow(clippy::expect_used, clippy::panic)]

use std::io::{Cursor, Write};
use std::sync::Arc;

use axum::Router;
use chrono::{DateTime, Utc};
use connectrpc::client::{ClientConfig, HttpClient};
use novasound_application::top_artists::{HistoryFormat, TopArtistsService};
use novasound_domain::top_artists::{
    DateRange, NativePeriod, NativeRanking, Provider, RankedArtist, TopArtistsError,
    TopArtistsProvider, UserId,
};
use novasound_storage_postgres::top_artists::TopArtistsStore;
use serde_json::json;
use tokio::sync::Notify;
use uuid::Uuid;

use crate::adapters::connect::{
    create_connect_router,
    top_artists_service::{ConnectTopArtistsService, map_error},
};
use crate::rpc::novasound::top_artists::v1::{self as proto, UserTopArtistsServiceExt};
use crate::state::AppState;

fn user(id: u128) -> UserId {
    UserId::new(Uuid::from_u128(id)).unwrap()
}
fn date(value: &str) -> DateTime<Utc> {
    value.parse().unwrap()
}
fn range() -> DateRange {
    DateRange::new(date("2026-09-03T00:00:00Z"), date("2026-09-05T00:00:00Z")).unwrap()
}

fn event(name: &str, time: &str, ms: i64) -> serde_json::Value {
    json!({"ts":time,"ms_played":ms,"master_metadata_track_name":"Track", "master_metadata_album_artist_name":name,"spotify_track_uri":format!("spotify:track:{name}"), "ip_addr":"discard-me"})
}

fn history() -> Vec<u8> {
    serde_json::to_vec(&vec![
        event("A", "2026-09-01T00:00:00Z", 10000),
        event("B", "2026-09-01T00:00:00Z", 10000),
        event("B", "2026-09-02T00:00:00Z", 10000),
        event("A", "2026-09-03T00:00:00Z", 0),
        event("A", "2026-09-03T01:00:00Z", 29999),
        event("A", "2026-09-03T02:00:00Z", 30000),
        event("B", "2026-09-04T00:00:00Z", 1_000_000),
        event("A", "2026-09-05T00:00:00Z", 30000),
    ])
    .unwrap()
}

#[derive(Clone)]
struct MockProvider {
    names: Vec<&'static str>,
}

impl TopArtistsProvider for MockProvider {
    fn fetch_top_artists(
        &self,
        user_id: UserId,
        _period: NativePeriod,
    ) -> impl std::future::Future<Output = Result<NativeRanking, TopArtistsError>> + Send {
        std::future::ready(Ok(NativeRanking {
            user_id,
            provider: Provider::Spotify,
            artists: self
                .names
                .iter()
                .map(|name| {
                    RankedArtist::from_provider(
                        user_id,
                        Provider::Spotify,
                        Some((*name).into()),
                        (*name).into(),
                        Some("https://example.test/image".into()),
                    )
                })
                .collect(),
        }))
    }
}

#[tokio::test]
async fn top_artists_import_aggregates_counts_boundaries_and_previous_window() {
    let pool = crate::get_test_pool!();
    let service = TopArtistsService::new(TopArtistsStore::new(pool.clone()));
    let summary = service
        .import_spotify_history(user(1), history(), HistoryFormat::Json)
        .await
        .unwrap();
    assert_eq!(summary.inserted, 8);
    assert_eq!(summary.duplicates, 0);
    let again = service
        .import_spotify_history(user(1), history(), HistoryFormat::Json)
        .await
        .unwrap();
    assert_eq!(again.inserted, 0);
    assert_eq!(again.duplicates, 8);
    let top = service
        .custom(user(1), Provider::Spotify, range(), 50)
        .await
        .unwrap();
    assert_eq!(top.entries.len(), 2);
    let a = &top.entries[0];
    assert_eq!(a.artist.name, "A");
    assert_eq!(a.rank, 1);
    assert_eq!(a.rank_change, Some(1));
    let metrics = a.metrics.as_ref().unwrap();
    assert_eq!(metrics.play_count, 3);
    assert_eq!(metrics.played_ms, 59999);
    assert_eq!(metrics.streams_30s, 1);
    assert_eq!(metrics.first_play, date("2026-09-01T00:00:00Z"));
    assert_eq!(metrics.last_play, date("2026-09-05T00:00:00Z"));
    assert_eq!(top.entries[1].rank_change, Some(-1));
    assert!(a.artist.image_url.is_none());
    assert!(a.artist.providers[0].artist_id.is_none());
    assert_eq!(top.history_start, Some(metrics.first_play));
    assert_eq!(top.history_end, Some(metrics.last_play));
    assert_eq!(top.comparison_at, Some(date("2026-09-01T00:00:00Z")));
    assert_eq!(
        service
            .custom(user(1), Provider::Spotify, range(), 1)
            .await
            .unwrap()
            .entries
            .len(),
        1
    );
    let stored: serde_json::Value = pool
        .get()
        .await
        .unwrap()
        .query_one("SELECT artist FROM top_artists_identities LIMIT 1", &[])
        .await
        .unwrap()
        .get(0);
    assert!(!stored.to_string().contains("discard-me"));
}

#[tokio::test]
async fn top_artists_isolates_users_and_concurrent_deduplication() {
    let pool = crate::get_test_pool!();
    let service = TopArtistsService::new(TopArtistsStore::new(pool));
    let empty = service
        .custom(user(2), Provider::Spotify, range(), 50)
        .await
        .unwrap();
    assert!(empty.entries.is_empty());
    assert!(empty.history_start.is_none());
    assert!(empty.history_end.is_none());
    let (first, second) = tokio::join!(
        service.import_spotify_history(user(1), history(), HistoryFormat::Json),
        service.import_spotify_history(user(1), history(), HistoryFormat::Json)
    );
    let first = first.unwrap();
    let second = second.unwrap();
    assert_eq!(first.inserted + second.inserted, 8);
    assert_eq!(first.duplicates + second.duplicates, 8);
    assert!(
        service
            .custom(user(2), Provider::Spotify, range(), 50)
            .await
            .unwrap()
            .entries
            .is_empty()
    );
    assert_eq!(
        service
            .import_spotify_history(user(2), history(), HistoryFormat::Json)
            .await
            .unwrap()
            .inserted,
        8
    );
    let one = service
        .custom(user(1), Provider::Spotify, range(), 50)
        .await
        .unwrap();
    let two = service
        .custom(user(2), Provider::Spotify, range(), 50)
        .await
        .unwrap();
    assert_ne!(one.entries[0].artist.id, two.entries[0].artist.id);
    assert_eq!(one.entries[0].metrics, two.entries[0].metrics);
}

#[tokio::test]
async fn top_artists_import_rejects_invalid_batch_without_partial_writes() {
    let pool = crate::get_test_pool!();
    let service = TopArtistsService::new(TopArtistsStore::new(pool));
    let invalid = serde_json::to_vec(&vec![
        event("Valid", "2026-09-04T00:00:00Z", 100),
        event("Invalid", "2026-09-04T00:00:00Z", -1),
    ])
    .unwrap();
    assert!(matches!(
        service
            .import_spotify_history(user(1), invalid, HistoryFormat::Json)
            .await,
        Err(TopArtistsError::InvalidImport)
    ));
    assert!(
        service
            .custom(user(1), Provider::Spotify, range(), 50)
            .await
            .unwrap()
            .entries
            .is_empty()
    );
    let repeated = event("A", "2026-09-04T00:00:00Z", 10);
    let input = serde_json::to_vec(&vec![repeated.clone(), repeated]).unwrap();
    let summary = service
        .import_spotify_history(user(1), input, HistoryFormat::Json)
        .await
        .unwrap();
    assert_eq!((summary.inserted, summary.duplicates), (1, 1));
    let top = service
        .custom(user(1), Provider::Spotify, range(), 50)
        .await
        .unwrap();
    assert_eq!(top.entries[0].rank_change, None);
    assert!(matches!(
        service
            .custom(user(1), Provider::Spotify, range(), 1001)
            .await,
        Err(TopArtistsError::InvalidInput(_))
    ));
}

#[tokio::test]
async fn top_artists_imports_zip_through_service_and_deduplicates_against_json() {
    let pool = crate::get_test_pool!();
    let service = TopArtistsService::new(TopArtistsStore::new(pool));
    let repeated = event("A", "2026-09-03T02:00:00Z", 30000);
    let initial = serde_json::to_vec(&vec![repeated.clone()]).unwrap();
    assert_eq!(
        service
            .import_spotify_history(user(1), initial, HistoryFormat::Json)
            .await
            .unwrap()
            .inserted,
        1
    );
    let new = event("A", "2026-09-04T02:00:00Z", 40000);
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    zip.start_file(
        "Streaming_History_Audio_2026_0.json",
        zip::write::SimpleFileOptions::default(),
    )
    .unwrap();
    zip.write_all(&serde_json::to_vec(&vec![repeated, new]).unwrap())
        .unwrap();
    let summary = service
        .import_spotify_history(
            user(1),
            zip.finish().unwrap().into_inner(),
            HistoryFormat::Zip,
        )
        .await
        .unwrap();
    assert_eq!(
        (summary.inserted, summary.duplicates, summary.skipped),
        (1, 1, 0)
    );
    let ranking = service
        .custom(user(1), Provider::Spotify, range(), 50)
        .await
        .unwrap();
    assert_eq!(ranking.entries.len(), 1);
    assert_eq!(ranking.entries[0].artist.name, "A");
    let metrics = ranking.entries[0].metrics.as_ref().unwrap();
    assert_eq!(
        (metrics.play_count, metrics.played_ms, metrics.streams_30s),
        (2, 70000, 2)
    );
}

struct DelayedProvider {
    started: Arc<Notify>,
    release: Arc<Notify>,
    names: Vec<&'static str>,
}

impl TopArtistsProvider for DelayedProvider {
    async fn fetch_top_artists(
        &self,
        user_id: UserId,
        period: NativePeriod,
    ) -> Result<NativeRanking, TopArtistsError> {
        self.started.notify_one();
        self.release.notified().await;
        MockProvider {
            names: self.names.clone(),
        }
        .fetch_top_artists(user_id, period)
        .await
    }
}

#[tokio::test]
async fn top_artists_rejects_older_service_request_completed_after_newer_one() {
    let pool = crate::get_test_pool!();
    let service = TopArtistsService::new(TopArtistsStore::new(pool.clone()));
    let old_started = Arc::new(Notify::new());
    let old_release = Arc::new(Notify::new());
    let older = DelayedProvider {
        started: old_started.clone(),
        release: old_release.clone(),
        names: vec!["Old"],
    };
    let old_service = service.clone();
    let old_task = tokio::spawn(async move {
        old_service
            .native(user(1), &older, NativePeriod::ShortTerm, 50)
            .await
    });
    old_started.notified().await;
    let new_started = Arc::new(Notify::new());
    let new_release = Arc::new(Notify::new());
    let newer = DelayedProvider {
        started: new_started.clone(),
        release: new_release.clone(),
        names: vec!["New"],
    };
    let new_service = service.clone();
    let new_task = tokio::spawn(async move {
        new_service
            .native(user(1), &newer, NativePeriod::ShortTerm, 50)
            .await
    });
    new_started.notified().await;
    new_release.notify_one();
    let saved = new_task.await.unwrap().unwrap();
    old_release.notify_one();
    assert!(matches!(
        old_task.await.unwrap(),
        Err(TopArtistsError::StaleSynchronization)
    ));
    let ranking: serde_json::Value = pool.get().await.unwrap()
        .query_one("SELECT ranking FROM top_artists_native_snapshots WHERE user_id = $1 AND provider = 'spotify' AND period = 'short_term'", &[&user(1).as_uuid()]).await.unwrap().get(0);
    assert_eq!(ranking["entries"][0]["artist"]["name"], "New");
    assert_eq!(ranking["observed_at"], serde_json::json!(saved.observed_at));
}

#[tokio::test]
async fn top_artists_native_compares_full_snapshots_and_keeps_periods_users_separate() {
    let pool = crate::get_test_pool!();
    let store = TopArtistsStore::new(pool);
    let service = TopArtistsService::new(store.clone());
    let provider = MockProvider {
        names: vec!["A", "B"],
    };
    let first = service
        .native(user(1), &provider, NativePeriod::ShortTerm, 1)
        .await
        .unwrap();
    assert_eq!(first.entries.len(), 1);
    assert!(first.entries[0].metrics.is_none());
    assert!(first.entries[0].rank_change.is_none());
    assert!(first.history_start.is_none());
    let reversed = MockProvider {
        names: vec!["B", "A"],
    };
    let second = service
        .native(user(1), &reversed, NativePeriod::ShortTerm, 50)
        .await
        .unwrap();
    assert_eq!(second.entries[0].rank_change, Some(1));
    assert_eq!(second.entries[1].rank_change, Some(-1));
    assert_eq!(second.comparison_at, Some(first.observed_at));
    let other = service
        .native(user(2), &reversed, NativePeriod::ShortTerm, 50)
        .await
        .unwrap();
    assert!(other.comparison_at.is_none());
    assert_ne!(other.entries[0].artist.id, second.entries[0].artist.id);
    let medium = service
        .native(user(1), &reversed, NativePeriod::MediumTerm, 50)
        .await
        .unwrap();
    assert!(medium.comparison_at.is_none());
    let stale = provider
        .fetch_top_artists(user(1), NativePeriod::ShortTerm)
        .await
        .unwrap();
    assert!(matches!(
        store
            .save_native(user(1), NativePeriod::ShortTerm, first.observed_at, stale)
            .await,
        Err(TopArtistsError::StaleSynchronization)
    ));
    let wrong_owner = provider
        .fetch_top_artists(user(2), NativePeriod::LongTerm)
        .await
        .unwrap();
    assert!(matches!(
        store
            .save_native(user(1), NativePeriod::LongTerm, Utc::now(), wrong_owner)
            .await,
        Err(TopArtistsError::AccessDenied)
    ));
}

async fn spawn(
    app: Router,
) -> (
    proto::UserTopArtistsServiceClient<HttpClient>,
    tokio::task::JoinHandle<()>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let uri = format!("http://{}", listener.local_addr().unwrap())
        .parse()
        .unwrap();
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (
        proto::UserTopArtistsServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri)),
        task,
    )
}

fn native_request() -> proto::GetTopArtistsRequest {
    proto::GetTopArtistsRequest {
        provider: proto::Provider::PROVIDER_SPOTIFY.into(),
        period: Some(
            proto::__buffa::oneof::get_top_artists_request::Period::NativePeriod(
                proto::NativePeriod::NATIVE_PERIOD_SHORT_TERM.into(),
            ),
        ),
        ..Default::default()
    }
}

#[tokio::test]
async fn top_artists_connect_round_trip_and_transport_validation() {
    let pool = crate::get_test_pool!();
    let service = TopArtistsService::new(TopArtistsStore::new(pool));
    let binding = ConnectTopArtistsService::for_internal_user(
        service,
        user(1),
        Some(MockProvider { names: vec!["A"] }),
    );
    let app = Router::new().fallback_service(
        Arc::new(binding)
            .register(connectrpc::Router::new())
            .into_axum_service(),
    );
    let (client, task) = spawn(app).await;
    let native = client
        .get_top_artists(native_request())
        .await
        .unwrap()
        .into_owned();
    assert_eq!(
        native.source,
        proto::RankingSource::RANKING_SOURCE_NATIVE_AFFINITY
    );
    assert!(native.entries[0].metrics.is_unset());
    assert!(native.entries[0].rank_change.is_none());
    let imported = client
        .import_listening_history(proto::ImportListeningHistoryRequest {
            provider: proto::Provider::PROVIDER_SPOTIFY.into(),
            format: proto::HistoryFormat::HISTORY_FORMAT_JSON.into(),
            content: history(),
            ..Default::default()
        })
        .await
        .unwrap()
        .into_owned();
    assert_eq!(imported.inserted, 8);
    let custom = client
        .get_top_artists(proto::GetTopArtistsRequest {
            provider: proto::Provider::PROVIDER_SPOTIFY.into(),
            period: Some(
                proto::__buffa::oneof::get_top_artists_request::Period::CustomPeriod(Box::new(
                    proto::DateRange {
                        start: "2026-09-03T00:00:00Z".into(),
                        end: "2026-09-05T00:00:00Z".into(),
                        ..Default::default()
                    },
                )),
            ),
            ..Default::default()
        })
        .await
        .unwrap()
        .into_owned();
    assert_eq!(custom.entries[0].metrics.as_option().unwrap().play_count, 3);
    assert_eq!(custom.entries[0].rank_change, Some(1));
    assert_eq!(
        custom.source,
        proto::RankingSource::RANKING_SOURCE_IMPORTED_HISTORY
    );
    let native_json = serde_json::to_value(&native).unwrap();
    assert!(native_json["entries"][0].get("metrics").is_none());
    let custom_json = serde_json::to_value(&custom).unwrap();
    assert_eq!(custom_json["entries"][0]["metrics"]["playCount"], "3");

    for request in [
        proto::GetTopArtistsRequest::default(),
        proto::GetTopArtistsRequest {
            period: None,
            ..native_request()
        },
        proto::GetTopArtistsRequest {
            limit: 1001,
            ..native_request()
        },
    ] {
        let error = client.get_top_artists(request).await.unwrap_err();
        assert_eq!(error.code, connectrpc::ErrorCode::InvalidArgument);
        assert_eq!(
            error.details[0].type_url,
            "type.novasound.dev/top-artists-error"
        );
    }
    let invalid = client
        .import_listening_history(proto::ImportListeningHistoryRequest {
            provider: proto::Provider::PROVIDER_SPOTIFY.into(),
            format: proto::HistoryFormat::HISTORY_FORMAT_JSON.into(),
            content: b"[] invalid".to_vec(),
            ..Default::default()
        })
        .await
        .unwrap_err();
    assert_eq!(invalid.code, connectrpc::ErrorCode::InvalidArgument);
    task.abort();
}

#[tokio::test]
async fn top_artists_is_not_exposed_by_the_production_router() {
    let pool = crate::get_test_pool!();
    let app = Router::new()
        .fallback_service(create_connect_router(AppState::new(pool)).into_axum_service());
    let (client, task) = spawn(app).await;
    let error = client.get_top_artists(native_request()).await.unwrap_err();
    assert_eq!(error.code, connectrpc::ErrorCode::Unimplemented);
    task.abort();
}

#[test]
fn top_artists_errors_are_stable_and_redacted() {
    let cases = [
        (
            TopArtistsError::ConnectionRequired,
            connectrpc::ErrorCode::FailedPrecondition,
            "connection_required",
        ),
        (
            TopArtistsError::AccessDenied,
            connectrpc::ErrorCode::PermissionDenied,
            "access_denied",
        ),
        (
            TopArtistsError::RateLimited {
                retry_after_seconds: Some(42),
            },
            connectrpc::ErrorCode::ResourceExhausted,
            "rate_limited",
        ),
        (
            TopArtistsError::InvalidProviderResponse,
            connectrpc::ErrorCode::Unavailable,
            "invalid_provider_response",
        ),
        (
            TopArtistsError::ProviderUnavailable,
            connectrpc::ErrorCode::Unavailable,
            "provider_unavailable",
        ),
        (
            TopArtistsError::ImportTooLarge,
            connectrpc::ErrorCode::ResourceExhausted,
            "import_too_large",
        ),
        (
            TopArtistsError::StaleSynchronization,
            connectrpc::ErrorCode::Aborted,
            "stale_synchronization",
        ),
        (
            TopArtistsError::Storage(Box::new(std::io::Error::other("private database details"))),
            connectrpc::ErrorCode::Internal,
            "internal",
        ),
    ];
    for (error, expected, code) in cases {
        let error = map_error(error);
        assert_eq!(error.code, expected);
        assert_eq!(error.details[0].debug.as_ref().unwrap()["code"], code);
        assert!(!error.to_string().contains("private"));
        if code == "rate_limited" {
            assert_eq!(
                error.details[0].debug.as_ref().unwrap()["retry_after_seconds"],
                42
            );
        }
    }
}
