//! User-owned Top Artists models. No provider wire formats or credentials belong here.
use std::future::Future;

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(value: Uuid) -> Result<Self, TopArtistsError> {
        if value.is_nil() {
            return Err(TopArtistsError::InvalidInput("user_id must not be nil"));
        }
        Ok(Self(value))
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    Spotify,
}

impl Provider {
    pub fn key(self) -> &'static str {
        match self {
            | Self::Spotify => "spotify",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativePeriod {
    ShortTerm,
    MediumTerm,
    LongTerm,
}

impl NativePeriod {
    pub fn key(self) -> &'static str {
        match self {
            | Self::ShortTerm => "short_term",
            | Self::MediumTerm => "medium_term",
            | Self::LongTerm => "long_term",
        }
    }
}

/// UTC half-open window [start, end). Validation includes the preceding comparison window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, TopArtistsError> {
        if start >= end || !supported_timestamp(start) || !supported_timestamp(end) {
            return Err(TopArtistsError::InvalidInput("invalid date range"));
        }
        let range = Self { start, end };
        if range.previous_start()?.year() < 1970 {
            return Err(TopArtistsError::InvalidInput("comparison precedes 1970"));
        }
        Ok(range)
    }

    pub fn start(self) -> DateTime<Utc> {
        self.start
    }
    pub fn end(self) -> DateTime<Utc> {
        self.end
    }

    pub fn previous_start(self) -> Result<DateTime<Utc>, TopArtistsError> {
        self.start
            .checked_sub_signed(self.end - self.start)
            .ok_or(TopArtistsError::InvalidInput("comparison range overflow"))
    }
}

/// PostgreSQL timestamp precision, excluding leap-second representations it cannot preserve.
pub fn supported_timestamp(date: DateTime<Utc>) -> bool {
    (1970..=9999).contains(&date.year())
        && date.timestamp_subsec_nanos() < 1_000_000_000
        && date.timestamp_subsec_nanos().is_multiple_of(1000)
}

pub fn validate_limit(limit: u32) -> Result<(), TopArtistsError> {
    if !(1..=1000).contains(&limit) {
        return Err(TopArtistsError::InvalidInput(
            "limit must be between 1 and 1000",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAssociation {
    pub provider: Provider,
    /// Absent for ESH name-only identities; never fabricated from a name.
    pub artist_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankedArtist {
    pub id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub providers: Vec<ProviderAssociation>,
}

impl RankedArtist {
    /// Stable user-scoped identity, with separate namespaces for resolved IDs and export names.
    pub fn from_provider(
        user: UserId,
        provider: Provider,
        provider_id: Option<String>,
        name: String,
        image_url: Option<String>,
    ) -> Self {
        let key = match &provider_id {
            | Some(id) => format!("{}:id:{id}", provider.key()),
            | None => format!("{}:export-name:{name}", provider.key()),
        };
        Self {
            id: Uuid::new_v5(&user.as_uuid(), key.as_bytes()),
            name,
            image_url,
            providers: vec![ProviderAssociation {
                provider,
                artist_id: provider_id,
            }],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListeningMetrics {
    pub play_count: i64,
    pub played_ms: i64,
    pub streams_30s: i64,
    pub first_play: DateTime<Utc>,
    pub last_play: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    NativeAffinity,
    ImportedHistory,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopEntry {
    pub rank: i64,
    pub artist: RankedArtist,
    pub metrics: Option<ListeningMetrics>,
    /// Previous rank minus current rank; absent without a comparable prior entry.
    pub rank_change: Option<i64>,
    pub source: Source,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopArtists {
    pub entries: Vec<TopEntry>,
    pub provider: Provider,
    pub source: Source,
    pub observed_at: DateTime<Utc>,
    pub comparison_at: Option<DateTime<Utc>>,
    /// Bounds of imported events, NOT a claim of complete listening history.
    pub history_start: Option<DateTime<Utc>>,
    pub history_end: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct ListeningEvent {
    pub fingerprint: String,
    pub artist: RankedArtist,
    pub ended_at: DateTime<Utc>,
    pub played_ms: i64,
}

#[derive(Debug)]
pub struct ImportedHistory {
    pub user_id: UserId,
    pub provider: Provider,
    pub events: Vec<ListeningEvent>,
    pub skipped: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportSummary {
    pub inserted: u64,
    pub duplicates: u64,
    pub skipped: u64,
}

#[derive(Debug)]
pub struct NativeRanking {
    pub user_id: UserId,
    pub provider: Provider,
    pub artists: Vec<RankedArtist>,
}

/// Narrow adapter seam for native Top Artists, not a general provider SDK.
pub trait TopArtistsProvider: Send + Sync {
    fn fetch_top_artists(
        &self,
        user_id: UserId,
        period: NativePeriod,
    ) -> impl Future<Output = Result<NativeRanking, TopArtistsError>> + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum TopArtistsError {
    #[error("Invalid input: {0}")]
    InvalidInput(&'static str),
    #[error("Provider connection required or expired")]
    ConnectionRequired,
    #[error("Provider access denied")]
    AccessDenied,
    #[error("Provider rate limited")]
    RateLimited { retry_after_seconds: Option<u64> },
    #[error("Provider unavailable")]
    ProviderUnavailable,
    #[error("Invalid provider response")]
    InvalidProviderResponse,
    #[error("Import exceeds supported limits")]
    ImportTooLarge,
    #[error("History import capacity is busy")]
    ImportBusy,
    #[error("Invalid extended streaming history")]
    InvalidImport,
    #[error("A newer synchronization has already completed")]
    StaleSynchronization,
    #[error("Top Artists storage failure")]
    Storage(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Top Artists worker failure")]
    Worker,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periods_validate_and_compare_equal_windows() {
        let start = "2026-09-01T00:00:00Z".parse().unwrap();
        let end = "2026-09-03T00:00:00Z".parse().unwrap();
        let range = DateRange::new(start, end).unwrap();
        assert_eq!(
            range.previous_start().unwrap(),
            "2026-08-30T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert!(DateRange::new(end, start).is_err());
        assert!(DateRange::new(start, start).is_err());
        assert!(DateRange::new("1970-01-01T00:00:00Z".parse().unwrap(), end).is_err());
        assert!(UserId::new(Uuid::nil()).is_err());
        assert!(supported_timestamp(
            "2026-09-01T00:00:00.000001Z".parse().unwrap()
        ));
        assert!(!supported_timestamp(
            "2026-09-01T00:00:00.0000001Z".parse().unwrap()
        ));
        assert!(!supported_timestamp(
            "2016-12-31T23:59:60Z".parse().unwrap()
        ));
    }

    #[test]
    fn identities_do_not_merge_users_or_names_with_provider_ids() {
        let one = UserId::new(Uuid::from_u128(1)).unwrap();
        let two = UserId::new(Uuid::from_u128(2)).unwrap();
        let artist = |user, id| {
            RankedArtist::from_provider(user, Provider::Spotify, id, "Same name".into(), None)
        };
        assert_ne!(artist(one, None).id, artist(two, None).id);
        assert_ne!(
            artist(one, None).id,
            artist(one, Some("Same name".into())).id
        );
        assert_eq!(artist(one, None).id, artist(one, None).id);
    }
}
