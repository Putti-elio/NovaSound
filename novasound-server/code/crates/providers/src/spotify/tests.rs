use super::*;
use axum::{
    Json, Router,
    extract::Query,
    http::{HeaderMap, StatusCode},
    routing::get,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn accepts_empty_rankings_and_maps_connection_failures() {
    let app = Router::new().route(
        "/me/top/artists",
        get(|| async { Json(json!({"offset":0,"total":0,"next":null,"items":[]})) }),
    );
    let (provider, task) = adapter(app).await;
    assert!(
        provider
            .fetch_top_artists(user(1), NativePeriod::ShortTerm)
            .await
            .unwrap()
            .artists
            .is_empty()
    );
    task.abort();
    let _ = task.await;
    assert!(matches!(
        provider
            .fetch_top_artists(user(1), NativePeriod::ShortTerm)
            .await,
        Err(TopArtistsError::ProviderUnavailable)
    ));
}

fn user(id: u128) -> UserId {
    UserId::new(Uuid::from_u128(id)).unwrap()
}

async fn adapter(app: Router) -> (SpotifyTopArtists, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mut provider =
        SpotifyTopArtists::new(SpotifyConnection::new(user(1), "secret").unwrap()).unwrap();
    provider.endpoint = format!("http://{}/me/top/artists", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (provider, task)
}

#[tokio::test]
async fn maps_and_paginates_all_native_periods_without_following_next_urls() {
    for period in [
        NativePeriod::ShortTerm,
        NativePeriod::MediumTerm,
        NativePeriod::LongTerm,
    ] {
        let expected = period.key();
        let app = Router::new().route("/me/top/artists", get(move |Query(query): Query<HashMap<String, String>>, headers: HeaderMap| async move {
            assert_eq!(headers["authorization"], "Bearer secret");
            assert_eq!(query["time_range"], expected);
            assert_eq!(query["limit"], "50");
            let offset: usize = query["offset"].parse().unwrap();
            assert!(offset == 0 || offset == 50, "unexpected offset: {offset}");
            let items: Vec<_> = (offset..if offset == 0 { 50 } else { 51 })
                .map(|index| json!({"id":format!("artist{index}"),"name":"An artist","images":[{"url":"https://i.scdn.co/image/test"}]}))
                .collect();
            Json(json!({"offset":offset, "total":51, "next": if offset == 0 {Some("https://evil.invalid/token-theft")} else {None}, "items":items}))
        }));
        let (provider, task) = adapter(app).await;
        let result = provider.fetch_top_artists(user(1), period).await.unwrap();
        assert_eq!(result.user_id, user(1));
        assert_eq!(result.artists.len(), 51);
        assert_eq!(
            result.artists[50].providers[0].artist_id.as_deref(),
            Some("artist50")
        );
        for (index, artist) in result.artists.iter().enumerate() {
            assert_eq!(
                artist.providers[0].artist_id.as_deref(),
                Some(format!("artist{index}").as_str())
            );
        }
        assert_eq!(
            result.artists[0].image_url.as_deref(),
            Some("https://i.scdn.co/image/test")
        );
        assert!(matches!(
            provider.fetch_top_artists(user(2), period).await,
            Err(TopArtistsError::AccessDenied)
        ));
        task.abort();
    }
}

#[tokio::test]
async fn maps_http_errors_without_exposing_provider_bodies() {
    for status in [401, 403, 429, 500, 302, 400] {
        let app = Router::new().route(
            "/me/top/artists",
            get(move || async move {
                (
                    StatusCode::from_u16(status).unwrap(),
                    [("retry-after", "42")],
                    "private upstream body",
                )
            }),
        );
        let (provider, task) = adapter(app).await;
        let error = provider
            .fetch_top_artists(user(1), NativePeriod::ShortTerm)
            .await
            .unwrap_err();
        match status {
            | 401 => assert!(matches!(error, TopArtistsError::ConnectionRequired)),
            | 403 => assert!(matches!(error, TopArtistsError::AccessDenied)),
            | 429 => assert!(matches!(
                error,
                TopArtistsError::RateLimited {
                    retry_after_seconds: Some(42)
                }
            )),
            | 500 => assert!(matches!(error, TopArtistsError::ProviderUnavailable)),
            | _ => assert!(matches!(error, TopArtistsError::InvalidProviderResponse)),
        }
        assert!(!error.to_string().contains("private"));
        task.abort();
    }
}

#[tokio::test]
async fn rejects_malformed_and_nonprogressing_pages() {
    for page in [
        json!({}),
        json!({"offset":0,"total":2,"next":"x","items":[]}),
        json!({"offset":1,"total":1,"items":[]}),
        json!({"offset":0,"total":1,"items":[{"id":"","name":"bad"}]}),
        json!({"offset":0,"total":2,"items":[{"id":"a","name":"A"},{"id":"a","name":"A"}]}),
    ] {
        let app = Router::new().route(
            "/me/top/artists",
            get(move || {
                let page = page.clone();
                async move { Json(page) }
            }),
        );
        let (provider, task) = adapter(app).await;
        assert!(matches!(
            provider
                .fetch_top_artists(user(1), NativePeriod::ShortTerm)
                .await,
            Err(TopArtistsError::InvalidProviderResponse)
        ));
        task.abort();
    }
}
