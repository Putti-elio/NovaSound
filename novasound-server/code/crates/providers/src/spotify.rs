use std::{collections::HashSet, time::Duration};

use novasound_domain::top_artists::{
    NativePeriod, NativeRanking, Provider, RankedArtist, TopArtistsError, TopArtistsProvider,
    UserId,
};
use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderValue},
    redirect::Policy,
};
use serde::Deserialize;

pub mod history;

/// Supplied only by trusted account-linking code. No serialization or credential getters.
pub struct SpotifyConnection {
    user_id: UserId,
    authorization: HeaderValue,
}

impl SpotifyConnection {
    pub fn new(user_id: UserId, access_token: &str) -> Result<Self, TopArtistsError> {
        if access_token.is_empty()
            || access_token.len() > 8192
            || access_token.chars().any(char::is_whitespace)
        {
            return Err(TopArtistsError::ConnectionRequired);
        }
        let mut authorization = HeaderValue::from_str(&format!("Bearer {access_token}"))
            .map_err(|_| TopArtistsError::ConnectionRequired)?;
        authorization.set_sensitive(true);
        Ok(Self {
            user_id,
            authorization,
        })
    }
}

pub struct SpotifyTopArtists {
    client: Client,
    connection: SpotifyConnection,
    endpoint: String,
}

impl SpotifyTopArtists {
    pub fn new(connection: SpotifyConnection) -> Result<Self, TopArtistsError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .redirect(Policy::none())
            .build()
            .map_err(|_| TopArtistsError::ProviderUnavailable)?;
        Ok(Self {
            client,
            connection,
            endpoint: "https://api.spotify.com/v1/me/top/artists".into(),
        })
    }

    async fn fetch(
        &self,
        user_id: UserId,
        period: NativePeriod,
    ) -> Result<NativeRanking, TopArtistsError> {
        if user_id != self.connection.user_id {
            return Err(TopArtistsError::AccessDenied);
        }
        let mut artists = Vec::new();
        let mut seen = HashSet::new();
        let mut expected_total = None;
        loop {
            // Construct offsets ourselves. Never send credentials to a provider-supplied next URL.
            let offset = artists.len();
            let mut response = self
                .client
                .get(&self.endpoint)
                .header(AUTHORIZATION, self.connection.authorization.clone())
                .query(&[
                    ("time_range", period.key()),
                    ("limit", "50"),
                    ("offset", &offset.to_string()),
                ])
                .send()
                .await
                .map_err(|_| TopArtistsError::ProviderUnavailable)?;
            let status = response.status().as_u16();
            if status != 200 {
                return Err(match status {
                    | 401 => TopArtistsError::ConnectionRequired,
                    | 403 => TopArtistsError::AccessDenied,
                    | 429 => TopArtistsError::RateLimited {
                        retry_after_seconds: response
                            .headers()
                            .get("retry-after")
                            .and_then(|value| value.to_str().ok())
                            .and_then(|value| value.parse().ok()),
                    },
                    | 500..=599 => TopArtistsError::ProviderUnavailable,
                    | _ => TopArtistsError::InvalidProviderResponse,
                });
            }
            let mut bytes = Vec::new();
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|_| TopArtistsError::ProviderUnavailable)?
            {
                if bytes.len() + chunk.len() > 2 * 1024 * 1024 {
                    return Err(TopArtistsError::InvalidProviderResponse);
                }
                bytes.extend_from_slice(&chunk);
            }
            let page: Page = serde_json::from_slice(&bytes)
                .map_err(|_| TopArtistsError::InvalidProviderResponse)?;
            if page.offset != offset
                || page.total > 10_000
                || page.items.len() > 50
                || expected_total.is_some_and(|total| total != page.total)
                || offset + page.items.len() > page.total
                || (page.items.is_empty() && offset < page.total)
            {
                return Err(TopArtistsError::InvalidProviderResponse);
            }
            expected_total = Some(page.total);
            for artist in page.items {
                if artist.id.is_empty()
                    || artist.name.trim().is_empty()
                    || !seen.insert(artist.id.clone())
                {
                    return Err(TopArtistsError::InvalidProviderResponse);
                }
                let image_url = artist
                    .images
                    .into_iter()
                    .map(|image| image.url)
                    .find(|url| reqwest::Url::parse(url).is_ok_and(|url| url.scheme() == "https"));
                artists.push(RankedArtist::from_provider(
                    user_id,
                    Provider::Spotify,
                    Some(artist.id),
                    artist.name,
                    image_url,
                ));
            }
            if artists.len() == page.total {
                if page.next.is_some() {
                    return Err(TopArtistsError::InvalidProviderResponse);
                }
                break;
            }
            if page.next.is_none() {
                return Err(TopArtistsError::InvalidProviderResponse);
            }
        }
        Ok(NativeRanking {
            user_id,
            provider: Provider::Spotify,
            artists,
        })
    }
}

impl TopArtistsProvider for SpotifyTopArtists {
    async fn fetch_top_artists(
        &self,
        user_id: UserId,
        period: NativePeriod,
    ) -> Result<NativeRanking, TopArtistsError> {
        tokio::time::timeout(Duration::from_secs(60), self.fetch(user_id, period))
            .await
            .map_err(|_| TopArtistsError::ProviderUnavailable)?
    }
}

#[derive(Deserialize)]
struct Page {
    items: Vec<Artist>,
    offset: usize,
    total: usize,
    next: Option<String>,
}

#[derive(Deserialize)]
struct Artist {
    id: String,
    name: String,
    #[serde(default)]
    images: Vec<Image>,
}

#[derive(Deserialize)]
struct Image {
    url: String,
}

#[cfg(test)]
mod tests;
