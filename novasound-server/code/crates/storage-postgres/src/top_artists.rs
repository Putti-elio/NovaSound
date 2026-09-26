use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::{DateTime, Utc};
use deadpool_postgres::{Pool, Transaction};
use novasound_domain::top_artists::{
    DateRange, ImportSummary, ImportedHistory, ListeningMetrics, NativePeriod, NativeRanking,
    Provider, RankedArtist, Source, TopArtists, TopArtistsError, TopEntry, UserId, validate_limit,
};
use serde_json::json;
use tokio_postgres::IsolationLevel;

use crate::clorinde::queries::top_artists as sql;

#[derive(Clone)]
pub struct TopArtistsStore {
    pool: Pool,
}

fn storage(error: impl std::error::Error + Send + Sync + 'static) -> TopArtistsError {
    TopArtistsError::Storage(Box::new(error))
}

fn validate_artist(
    user: UserId,
    provider: Provider,
    artist: &RankedArtist,
) -> Result<(), TopArtistsError> {
    let [association] = artist.providers.as_slice() else {
        return Err(TopArtistsError::InvalidProviderResponse);
    };
    let expected = RankedArtist::from_provider(
        user,
        provider,
        association.artist_id.clone(),
        artist.name.clone(),
        artist.image_url.clone(),
    );
    if association.provider != provider || expected.id != artist.id || artist.name.trim().is_empty()
    {
        return Err(TopArtistsError::AccessDenied);
    }
    Ok(())
}

async fn lock_owner(
    transaction: &Transaction<'_>,
    user: UserId,
    provider: Provider,
) -> Result<(), TopArtistsError> {
    sql::ensure_owner()
        .bind(transaction, &user.as_uuid(), &provider.key())
        .await
        .map_err(storage)?;
    sql::lock_owner()
        .bind(transaction, &user.as_uuid(), &provider.key())
        .one()
        .await
        .map_err(storage)?;
    Ok(())
}

impl TopArtistsStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Atomic, retry-safe import; ownership is verified even for internally constructed input.
    pub async fn import(
        &self,
        user: UserId,
        history: ImportedHistory,
    ) -> Result<ImportSummary, TopArtistsError> {
        if user != history.user_id {
            return Err(TopArtistsError::AccessDenied);
        }
        let mut artists = BTreeMap::new();
        for event in &history.events {
            validate_artist(user, history.provider, &event.artist)?;
            if event.fingerprint.len() != 64
                || !event
                    .fingerprint
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit())
                || !(0..=i64::from(i32::MAX)).contains(&event.played_ms)
            {
                return Err(TopArtistsError::InvalidImport);
            }
            artists.insert(event.artist.id, &event.artist);
        }
        let mut client = self.pool.get().await.map_err(storage)?;
        let transaction = client.transaction().await.map_err(storage)?;
        lock_owner(&transaction, user, history.provider).await?;
        // Bounded batches avoid a database round-trip for every listening event.
        let identities: Vec<_> = artists
            .values()
            .map(|artist| json!({"id": artist.id, "artist": artist}))
            .collect();
        for chunk in identities.chunks(1000) {
            sql::upsert_identities()
                .bind(
                    &transaction,
                    &user.as_uuid(),
                    &history.provider.key(),
                    &json!(chunk),
                )
                .await
                .map_err(storage)?;
        }
        let mut inserted = 0_u64;
        for chunk in history.events.chunks(1000) {
            let events: Vec<_> = chunk
                .iter()
                .map(|event| {
                    json!({
                        "fingerprint": event.fingerprint, "artist_id": event.artist.id,
                        "ended_at": event.ended_at, "played_ms": event.played_ms,
                    })
                })
                .collect();
            let count = sql::insert_events()
                .bind(
                    &transaction,
                    &user.as_uuid(),
                    &history.provider.key(),
                    &json!(events),
                )
                .one()
                .await
                .map_err(storage)?;
            inserted += u64::try_from(count).map_err(storage)?;
        }
        transaction.commit().await.map_err(storage)?;
        Ok(ImportSummary {
            inserted,
            duplicates: history.events.len() as u64 - inserted,
            skipped: history.skipped,
        })
    }

    /// Serialize synchronizations per owner and reject results started before the saved snapshot.
    pub async fn save_native(
        &self,
        user: UserId,
        period: NativePeriod,
        observed_at: DateTime<Utc>,
        ranking: NativeRanking,
    ) -> Result<TopArtists, TopArtistsError> {
        if ranking.user_id != user {
            return Err(TopArtistsError::AccessDenied);
        }
        let mut seen = HashSet::new();
        for artist in &ranking.artists {
            validate_artist(user, ranking.provider, artist)?;
            if !seen.insert(artist.id) {
                return Err(TopArtistsError::InvalidProviderResponse);
            }
        }
        let mut client = self.pool.get().await.map_err(storage)?;
        let transaction = client.transaction().await.map_err(storage)?;
        lock_owner(&transaction, user, ranking.provider).await?;
        let previous: Option<TopArtists> = sql::get_snapshot()
            .bind(
                &transaction,
                &user.as_uuid(),
                &ranking.provider.key(),
                &period.key(),
            )
            .opt()
            .await
            .map_err(storage)?
            .map(serde_json::from_value)
            .transpose()
            .map_err(storage)?;
        if previous
            .as_ref()
            .is_some_and(|previous| previous.observed_at >= observed_at)
        {
            return Err(TopArtistsError::StaleSynchronization);
        }
        let previous_ranks: HashMap<_, _> = previous
            .iter()
            .flat_map(|top| &top.entries)
            .map(|entry| (entry.artist.id, entry.rank))
            .collect();
        let entries = ranking
            .artists
            .into_iter()
            .enumerate()
            .map(|(index, artist)| {
                let rank = i64::try_from(index + 1).map_err(storage)?;
                let rank_change = previous_ranks
                    .get(&artist.id)
                    .map(|previous| previous - rank);
                Ok(TopEntry {
                    rank,
                    artist,
                    metrics: None,
                    rank_change,
                    source: Source::NativeAffinity,
                })
            })
            .collect::<Result<Vec<_>, TopArtistsError>>()?;
        let result = TopArtists {
            entries,
            provider: ranking.provider,
            source: Source::NativeAffinity,
            observed_at,
            comparison_at: previous.as_ref().map(|previous| previous.observed_at),
            history_start: None,
            history_end: None,
        };
        sql::save_snapshot()
            .bind(
                &transaction,
                &user.as_uuid(),
                &ranking.provider.key(),
                &period.key(),
                &observed_at.fixed_offset(),
                &serde_json::to_value(&result).map_err(storage)?,
            )
            .await
            .map_err(storage)?;
        transaction.commit().await.map_err(storage)?;
        Ok(result)
    }

    pub async fn custom(
        &self,
        user: UserId,
        provider: Provider,
        range: DateRange,
        limit: u32,
    ) -> Result<TopArtists, TopArtistsError> {
        validate_limit(limit)?;
        let mut client = self.pool.get().await.map_err(storage)?;
        // Rankings and coverage must observe the same import snapshot.
        let transaction = client
            .build_transaction()
            .isolation_level(IsolationLevel::RepeatableRead)
            .read_only(true)
            .start()
            .await
            .map_err(storage)?;
        let rows = sql::custom_ranking()
            .bind(
                &transaction,
                &range.start().fixed_offset(),
                &range.end().fixed_offset(),
                &range.previous_start()?.fixed_offset(),
                &user.as_uuid(),
                &provider.key(),
                &i64::from(limit),
            )
            .all()
            .await
            .map_err(storage)?;
        let entries = rows
            .into_iter()
            .map(|row| {
                Ok(TopEntry {
                    rank: row.rank,
                    artist: serde_json::from_value(row.artist).map_err(storage)?,
                    metrics: Some(ListeningMetrics {
                        play_count: row.plays,
                        played_ms: row.played_ms,
                        streams_30s: row.streams,
                        first_play: row.first_play.to_utc(),
                        last_play: row.last_play.to_utc(),
                    }),
                    rank_change: row.rank_change,
                    source: Source::ImportedHistory,
                })
            })
            .collect::<Result<Vec<_>, TopArtistsError>>()?;
        let coverage = sql::history_coverage()
            .bind(&transaction, &user.as_uuid(), &provider.key())
            .one()
            .await
            .map_err(storage)?;
        transaction.commit().await.map_err(storage)?;
        Ok(TopArtists {
            entries,
            provider,
            source: Source::ImportedHistory,
            observed_at: Utc::now(),
            comparison_at: Some(range.previous_start()?),
            history_start: coverage.first_event.map(|date| date.to_utc()),
            history_end: coverage.last_event.map(|date| date.to_utc()),
        })
    }
}
