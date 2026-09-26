// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct EnsureOwnerParams<T1: crate::StringSql> {
    pub user_id: uuid::Uuid,
    pub provider: T1,
}
#[derive(Debug)]
pub struct LockOwnerParams<T1: crate::StringSql> {
    pub user_id: uuid::Uuid,
    pub provider: T1,
}
#[derive(Debug)]
pub struct UpsertIdentitiesParams<T1: crate::StringSql, T2: crate::JsonSql> {
    pub user_id: uuid::Uuid,
    pub provider: T1,
    pub artists: T2,
}
#[derive(Debug)]
pub struct InsertEventsParams<T1: crate::StringSql, T2: crate::JsonSql> {
    pub user_id: uuid::Uuid,
    pub provider: T1,
    pub events: T2,
}
#[derive(Debug)]
pub struct GetSnapshotParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub user_id: uuid::Uuid,
    pub provider: T1,
    pub period: T2,
}
#[derive(Debug)]
pub struct SaveSnapshotParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::JsonSql> {
    pub user_id: uuid::Uuid,
    pub provider: T1,
    pub period: T2,
    pub observed_at: chrono::DateTime<chrono::FixedOffset>,
    pub ranking: T3,
}
#[derive(Debug)]
pub struct HistoryCoverageParams<T1: crate::StringSql> {
    pub user_id: uuid::Uuid,
    pub provider: T1,
}
#[derive(Debug)]
pub struct CustomRankingParams<T1: crate::StringSql> {
    pub start_at: chrono::DateTime<chrono::FixedOffset>,
    pub end_at: chrono::DateTime<chrono::FixedOffset>,
    pub previous_start: chrono::DateTime<chrono::FixedOffset>,
    pub user_id: uuid::Uuid,
    pub provider: T1,
    pub result_limit: i64,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct HistoryCoverage {
    pub first_event: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub last_event: Option<chrono::DateTime<chrono::FixedOffset>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CustomRanking {
    pub rank: i64,
    pub artist: serde_json::Value,
    pub plays: i64,
    pub played_ms: i64,
    pub streams: i64,
    pub first_play: chrono::DateTime<chrono::FixedOffset>,
    pub last_play: chrono::DateTime<chrono::FixedOffset>,
    pub rank_change: Option<i64>,
}
pub struct CustomRankingBorrowed<'a> {
    pub rank: i64,
    pub artist: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub plays: i64,
    pub played_ms: i64,
    pub streams: i64,
    pub first_play: chrono::DateTime<chrono::FixedOffset>,
    pub last_play: chrono::DateTime<chrono::FixedOffset>,
    pub rank_change: Option<i64>,
}
impl<'a> From<CustomRankingBorrowed<'a>> for CustomRanking {
    fn from(
        CustomRankingBorrowed {
            rank,
            artist,
            plays,
            played_ms,
            streams,
            first_play,
            last_play,
            rank_change,
        }: CustomRankingBorrowed<'a>,
    ) -> Self {
        Self {
            rank,
            artist: serde_json::from_str(artist.0.get()).unwrap(),
            plays,
            played_ms,
            streams,
            first_play,
            last_play,
            rank_change,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UuidUuidQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<uuid::Uuid, tokio_postgres::Error>,
    mapper: fn(uuid::Uuid) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UuidUuidQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(uuid::Uuid) -> R) -> UuidUuidQuery<'c, 'a, 's, C, R, N> {
        UuidUuidQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct I64Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i64, tokio_postgres::Error>,
    mapper: fn(i64) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I64Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'c, 'a, 's, C, R, N> {
        I64Query {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct SerdejsonValueQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(
            &tokio_postgres::Row,
        )
            -> Result<postgres_types::Json<&serde_json::value::RawValue>, tokio_postgres::Error>,
    mapper: fn(postgres_types::Json<&serde_json::value::RawValue>) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SerdejsonValueQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(postgres_types::Json<&serde_json::value::RawValue>) -> R,
    ) -> SerdejsonValueQuery<'c, 'a, 's, C, R, N> {
        SerdejsonValueQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct HistoryCoverageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<HistoryCoverage, tokio_postgres::Error>,
    mapper: fn(HistoryCoverage) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> HistoryCoverageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(HistoryCoverage) -> R,
    ) -> HistoryCoverageQuery<'c, 'a, 's, C, R, N> {
        HistoryCoverageQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CustomRankingQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CustomRankingBorrowed, tokio_postgres::Error>,
    mapper: fn(CustomRankingBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CustomRankingQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CustomRankingBorrowed) -> R,
    ) -> CustomRankingQuery<'c, 'a, 's, C, R, N> {
        CustomRankingQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct EnsureOwnerStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn ensure_owner() -> EnsureOwnerStmt {
    EnsureOwnerStmt(
        "INSERT INTO top_artists_owners (user_id, provider) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        None,
    )
}
impl EnsureOwnerStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, provider]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        EnsureOwnerParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for EnsureOwnerStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a EnsureOwnerParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.provider))
    }
}
pub struct LockOwnerStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn lock_owner() -> LockOwnerStmt {
    LockOwnerStmt(
        "SELECT user_id FROM top_artists_owners WHERE user_id = $1 AND provider = $2 FOR UPDATE",
        None,
    )
}
impl LockOwnerStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
    ) -> UuidUuidQuery<'c, 'a, 's, C, uuid::Uuid, 2> {
        UuidUuidQuery {
            client,
            params: [user_id, provider],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        LockOwnerParams<T1>,
        UuidUuidQuery<'c, 'a, 's, C, uuid::Uuid, 2>,
        C,
    > for LockOwnerStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a LockOwnerParams<T1>,
    ) -> UuidUuidQuery<'c, 'a, 's, C, uuid::Uuid, 2> {
        self.bind(client, &params.user_id, &params.provider)
    }
}
pub struct UpsertIdentitiesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn upsert_identities() -> UpsertIdentitiesStmt {
    UpsertIdentitiesStmt(
        "INSERT INTO top_artists_identities (user_id, provider, id, artist) SELECT $1, $2, x.id, x.artist FROM jsonb_to_recordset($3::jsonb) AS x(id UUID, artist JSONB) ON CONFLICT (user_id, provider, id) DO UPDATE SET artist = EXCLUDED.artist",
        None,
    )
}
impl UpsertIdentitiesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::JsonSql>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
        artists: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, provider, artists]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::JsonSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpsertIdentitiesParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpsertIdentitiesStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpsertIdentitiesParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.provider, &params.artists))
    }
}
pub struct InsertEventsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_events() -> InsertEventsStmt {
    InsertEventsStmt(
        "WITH inserted AS ( INSERT INTO top_artists_events (user_id, provider, fingerprint, artist_id, ended_at, played_ms) SELECT $1, $2, x.fingerprint, x.artist_id, x.ended_at, x.played_ms FROM jsonb_to_recordset($3::jsonb) AS x(fingerprint TEXT, artist_id UUID, ended_at TIMESTAMPTZ, played_ms BIGINT) ON CONFLICT DO NOTHING RETURNING fingerprint ) SELECT COUNT(*) AS inserted FROM inserted",
        None,
    )
}
impl InsertEventsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::JsonSql>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
        events: &'a T2,
    ) -> I64Query<'c, 'a, 's, C, i64, 3> {
        I64Query {
            client,
            params: [user_id, provider, events],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::JsonSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        InsertEventsParams<T1, T2>,
        I64Query<'c, 'a, 's, C, i64, 3>,
        C,
    > for InsertEventsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a InsertEventsParams<T1, T2>,
    ) -> I64Query<'c, 'a, 's, C, i64, 3> {
        self.bind(client, &params.user_id, &params.provider, &params.events)
    }
}
pub struct GetSnapshotStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_snapshot() -> GetSnapshotStmt {
    GetSnapshotStmt(
        "SELECT ranking FROM top_artists_native_snapshots WHERE user_id = $1 AND provider = $2 AND period = $3",
        None,
    )
}
impl GetSnapshotStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
        period: &'a T2,
    ) -> SerdejsonValueQuery<'c, 'a, 's, C, serde_json::Value, 3> {
        SerdejsonValueQuery {
            client,
            params: [user_id, provider, period],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| serde_json::from_str(it.0.get()).unwrap(),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetSnapshotParams<T1, T2>,
        SerdejsonValueQuery<'c, 'a, 's, C, serde_json::Value, 3>,
        C,
    > for GetSnapshotStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetSnapshotParams<T1, T2>,
    ) -> SerdejsonValueQuery<'c, 'a, 's, C, serde_json::Value, 3> {
        self.bind(client, &params.user_id, &params.provider, &params.period)
    }
}
pub struct SaveSnapshotStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn save_snapshot() -> SaveSnapshotStmt {
    SaveSnapshotStmt(
        "INSERT INTO top_artists_native_snapshots (user_id, provider, period, observed_at, ranking) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (user_id, provider, period) DO UPDATE SET observed_at = EXCLUDED.observed_at, ranking = EXCLUDED.ranking",
        None,
    )
}
impl SaveSnapshotStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::JsonSql,
    >(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
        period: &'a T2,
        observed_at: &'a chrono::DateTime<chrono::FixedOffset>,
        ranking: &'a T3,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[user_id, provider, period, observed_at, ranking])
            .await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::JsonSql,
    >
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        SaveSnapshotParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for SaveSnapshotStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a SaveSnapshotParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.user_id,
            &params.provider,
            &params.period,
            &params.observed_at,
            &params.ranking,
        ))
    }
}
pub struct HistoryCoverageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn history_coverage() -> HistoryCoverageStmt {
    HistoryCoverageStmt(
        "SELECT MIN(ended_at) AS first_event, MAX(ended_at) AS last_event FROM top_artists_events WHERE user_id = $1 AND provider = $2",
        None,
    )
}
impl HistoryCoverageStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
    ) -> HistoryCoverageQuery<'c, 'a, 's, C, HistoryCoverage, 2> {
        HistoryCoverageQuery {
            client,
            params: [user_id, provider],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<HistoryCoverage, tokio_postgres::Error> {
                    Ok(HistoryCoverage {
                        first_event: row.try_get(0)?,
                        last_event: row.try_get(1)?,
                    })
                },
            mapper: |it| HistoryCoverage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        HistoryCoverageParams<T1>,
        HistoryCoverageQuery<'c, 'a, 's, C, HistoryCoverage, 2>,
        C,
    > for HistoryCoverageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a HistoryCoverageParams<T1>,
    ) -> HistoryCoverageQuery<'c, 'a, 's, C, HistoryCoverage, 2> {
        self.bind(client, &params.user_id, &params.provider)
    }
}
pub struct CustomRankingStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn custom_ranking() -> CustomRankingStmt {
    CustomRankingStmt(
        "WITH totals AS ( SELECT artist_id, COUNT(*) FILTER (WHERE ended_at >= $1 AND ended_at < $2) AS plays, COALESCE(SUM(played_ms) FILTER (WHERE ended_at >= $1 AND ended_at < $2), 0)::bigint AS played_ms, COUNT(*) FILTER (WHERE ended_at >= $1 AND ended_at < $2 AND played_ms >= 30000) AS streams, COUNT(*) FILTER (WHERE ended_at >= $3 AND ended_at < $1) AS previous_plays FROM top_artists_events WHERE user_id = $4 AND provider = $5 AND ended_at >= $3 AND ended_at < $2 GROUP BY artist_id ), current_ranks AS ( SELECT *, ROW_NUMBER() OVER (ORDER BY plays DESC, artist_id) AS rank FROM totals WHERE plays > 0 ), previous_ranks AS ( SELECT artist_id, ROW_NUMBER() OVER (ORDER BY previous_plays DESC, artist_id) AS rank FROM totals WHERE previous_plays > 0 ), current_page AS MATERIALIZED ( SELECT * FROM current_ranks ORDER BY rank LIMIT $6 ) SELECT c.rank, i.artist, c.plays, c.played_ms, c.streams, first_event.ended_at AS first_play, last_event.ended_at AS last_play, p.rank - c.rank AS rank_change FROM current_page c JOIN top_artists_identities i ON i.user_id = $4 AND i.provider = $5 AND i.id = c.artist_id LEFT JOIN previous_ranks p ON p.artist_id = c.artist_id JOIN LATERAL ( SELECT ended_at FROM top_artists_events WHERE user_id = $4 AND provider = $5 AND artist_id = c.artist_id ORDER BY ended_at ASC LIMIT 1 ) first_event ON TRUE JOIN LATERAL ( SELECT ended_at FROM top_artists_events WHERE user_id = $4 AND provider = $5 AND artist_id = c.artist_id ORDER BY ended_at DESC LIMIT 1 ) last_event ON TRUE ORDER BY c.rank",
        None,
    )
}
impl CustomRankingStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        start_at: &'a chrono::DateTime<chrono::FixedOffset>,
        end_at: &'a chrono::DateTime<chrono::FixedOffset>,
        previous_start: &'a chrono::DateTime<chrono::FixedOffset>,
        user_id: &'a uuid::Uuid,
        provider: &'a T1,
        result_limit: &'a i64,
    ) -> CustomRankingQuery<'c, 'a, 's, C, CustomRanking, 6> {
        CustomRankingQuery {
            client,
            params: [
                start_at,
                end_at,
                previous_start,
                user_id,
                provider,
                result_limit,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CustomRankingBorrowed, tokio_postgres::Error> {
                    Ok(CustomRankingBorrowed {
                        rank: row.try_get(0)?,
                        artist: row.try_get(1)?,
                        plays: row.try_get(2)?,
                        played_ms: row.try_get(3)?,
                        streams: row.try_get(4)?,
                        first_play: row.try_get(5)?,
                        last_play: row.try_get(6)?,
                        rank_change: row.try_get(7)?,
                    })
                },
            mapper: |it| CustomRanking::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CustomRankingParams<T1>,
        CustomRankingQuery<'c, 'a, 's, C, CustomRanking, 6>,
        C,
    > for CustomRankingStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CustomRankingParams<T1>,
    ) -> CustomRankingQuery<'c, 'a, 's, C, CustomRanking, 6> {
        self.bind(
            client,
            &params.start_at,
            &params.end_at,
            &params.previous_start,
            &params.user_id,
            &params.provider,
            &params.result_limit,
        )
    }
}
