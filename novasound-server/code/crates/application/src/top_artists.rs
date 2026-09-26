use std::sync::Arc;

use chrono::{DateTime, Utc};
use novasound_domain::top_artists::{
    DateRange, ImportSummary, ImportedHistory, NativePeriod, NativeRanking, Provider, TopArtists,
    TopArtistsError, TopArtistsProvider, UserId,
};
pub use novasound_providers::spotify::history::HistoryFormat;
use novasound_providers::spotify::history::{MAX_INPUT_BYTES, parse_history};
use novasound_storage_postgres::top_artists::TopArtistsStore;
use tokio::sync::Semaphore;

/// Application-owned persistence contract; adapters supply an implementation.
pub trait TopArtistsStorage: Clone + Send + Sync {
    fn save_native(
        &self,
        user: UserId,
        period: NativePeriod,
        observed_at: DateTime<Utc>,
        ranking: NativeRanking,
    ) -> impl Future<Output = Result<TopArtists, TopArtistsError>> + Send;
    fn custom(
        &self,
        user: UserId,
        provider: Provider,
        range: DateRange,
        limit: u32,
    ) -> impl Future<Output = Result<TopArtists, TopArtistsError>> + Send;
    fn import(
        &self,
        user: UserId,
        history: ImportedHistory,
    ) -> impl Future<Output = Result<ImportSummary, TopArtistsError>> + Send;
}

// Other existing application services still use PostgreSQL directly. Keep this adapter local
// until those unrelated services can move to ports without introducing a dependency cycle.
impl TopArtistsStorage for TopArtistsStore {
    async fn save_native(
        &self,
        user: UserId,
        period: NativePeriod,
        observed_at: DateTime<Utc>,
        ranking: NativeRanking,
    ) -> Result<TopArtists, TopArtistsError> {
        TopArtistsStore::save_native(self, user, period, observed_at, ranking).await
    }

    async fn custom(
        &self,
        user: UserId,
        provider: Provider,
        range: DateRange,
        limit: u32,
    ) -> Result<TopArtists, TopArtistsError> {
        TopArtistsStore::custom(self, user, provider, range, limit).await
    }

    async fn import(
        &self,
        user: UserId,
        history: ImportedHistory,
    ) -> Result<ImportSummary, TopArtistsError> {
        TopArtistsStore::import(self, user, history).await
    }
}

/// Share one instance across transports so decompression concurrency is bounded.
#[derive(Clone)]
pub struct TopArtistsService<S> {
    store: S,
    import_workers: Arc<Semaphore>,
}

impl<S: TopArtistsStorage> TopArtistsService<S> {
    pub fn new(store: S) -> Self {
        Self {
            store,
            import_workers: Arc::new(Semaphore::new(2)),
        }
    }

    pub async fn native(
        &self,
        user_id: UserId,
        provider: &impl TopArtistsProvider,
        period: NativePeriod,
        limit: u32,
    ) -> Result<TopArtists, TopArtistsError> {
        novasound_domain::top_artists::validate_limit(limit)?;
        let started_at = Utc::now();
        let ranking = provider.fetch_top_artists(user_id, period).await?;
        let mut result = self
            .store
            .save_native(user_id, period, started_at, ranking)
            .await?;
        // Store the full snapshot, not the caller's display limit, for meaningful rank changes.
        result.entries.truncate(limit as usize);
        Ok(result)
    }

    pub async fn custom(
        &self,
        user_id: UserId,
        provider: Provider,
        range: DateRange,
        limit: u32,
    ) -> Result<TopArtists, TopArtistsError> {
        novasound_domain::top_artists::validate_limit(limit)?;
        self.store.custom(user_id, provider, range, limit).await
    }

    pub async fn import_spotify_history(
        &self,
        user_id: UserId,
        bytes: Vec<u8>,
        format: HistoryFormat,
    ) -> Result<ImportSummary, TopArtistsError> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(TopArtistsError::ImportTooLarge);
        }
        let permit = self
            .import_workers
            .clone()
            .try_acquire_owned()
            .map_err(|_| TopArtistsError::ImportBusy)?;
        // Keep the permit through persistence; queued normalized histories also consume memory.
        // Moving it into the worker protects the limit if its awaiting request is cancelled.
        let (history, _permit) =
            tokio::task::spawn_blocking(move || (parse_history(user_id, &bytes, format), permit))
                .await
                .map_err(|_| TopArtistsError::Worker)?;
        self.store.import(user_id, history?).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[derive(Clone)]
    struct FailingStorage;

    impl TopArtistsStorage for FailingStorage {
        async fn save_native(
            &self,
            _: UserId,
            _: NativePeriod,
            _: DateTime<Utc>,
            _: NativeRanking,
        ) -> Result<TopArtists, TopArtistsError> {
            Err(TopArtistsError::Worker)
        }
        async fn custom(
            &self,
            _: UserId,
            _: Provider,
            _: DateRange,
            _: u32,
        ) -> Result<TopArtists, TopArtistsError> {
            Err(TopArtistsError::Worker)
        }
        async fn import(
            &self,
            _: UserId,
            _: ImportedHistory,
        ) -> Result<ImportSummary, TopArtistsError> {
            Err(TopArtistsError::Storage(Box::new(std::io::Error::other(
                "storage unavailable",
            ))))
        }
    }

    #[test]
    fn import_capacity_is_shared_and_released_after_errors() {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async {
                let service = TopArtistsService::new(FailingStorage);
                let user = UserId::new(Uuid::from_u128(1)).unwrap();
                let permits = service
                    .import_workers
                    .clone()
                    .try_acquire_many_owned(2)
                    .unwrap();
                assert!(matches!(
                    service
                        .clone()
                        .import_spotify_history(user, b"[]".to_vec(), HistoryFormat::Json)
                        .await,
                    Err(TopArtistsError::ImportBusy)
                ));
                drop(permits);
                for _ in 0..3 {
                    assert!(matches!(
                        service
                            .import_spotify_history(user, b"bad".to_vec(), HistoryFormat::Json)
                            .await,
                        Err(TopArtistsError::InvalidImport)
                    ));
                }
                for _ in 0..3 {
                    assert!(matches!(
                        service
                            .import_spotify_history(user, b"[]".to_vec(), HistoryFormat::Json)
                            .await,
                        Err(TopArtistsError::Storage(_))
                    ));
                }
            });
    }
}
