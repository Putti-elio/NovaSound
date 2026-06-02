// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertSongParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub id: T1,
    pub name: T2,
    pub duration: i32,
    pub artist_id: T3,
    pub album_id: Option<T4>,
    pub release_date: Option<i64>,
    pub track_number: Option<i32>,
    pub image_path: Option<T5>,
}
#[derive(Debug)]
pub struct UpdateSongNameParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub id: T2,
}
#[derive(Debug)]
pub struct UpdateSongDurationParams<T1: crate::StringSql> {
    pub duration: i32,
    pub id: T1,
}
#[derive(Debug)]
pub struct UpdateSongReleaseDateParams<T1: crate::StringSql> {
    pub release_date: i64,
    pub id: T1,
}
#[derive(Debug)]
pub struct UpdateSongTrackNumberParams<T1: crate::StringSql> {
    pub track_number: i32,
    pub id: T1,
}
#[derive(Debug)]
pub struct UpdateSongPartialParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub name: Option<T1>,
    pub duration: Option<i32>,
    pub artist_id: Option<T2>,
    pub album_id: Option<T3>,
    pub release_date: Option<i64>,
    pub track_number: Option<i32>,
    pub image_path: Option<T4>,
    pub id: T5,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub duration: i32,
    pub artist_id: String,
    pub album_id: Option<String>,
    pub release_date: Option<i64>,
    pub track_number: Option<i32>,
    pub image_path: Option<String>,
}
pub struct SongBorrowed<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub duration: i32,
    pub artist_id: &'a str,
    pub album_id: Option<&'a str>,
    pub release_date: Option<i64>,
    pub track_number: Option<i32>,
    pub image_path: Option<&'a str>,
}
impl<'a> From<SongBorrowed<'a>> for Song {
    fn from(
        SongBorrowed {
            id,
            name,
            duration,
            artist_id,
            album_id,
            release_date,
            track_number,
            image_path,
        }: SongBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            duration,
            artist_id: artist_id.into(),
            album_id: album_id.map(|v| v.into()),
            release_date,
            track_number,
            image_path: image_path.map(|v| v.into()),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct SongQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<SongBorrowed, tokio_postgres::Error>,
    mapper: fn(SongBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SongQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(SongBorrowed) -> R) -> SongQuery<'c, 'a, 's, C, R, N> {
        SongQuery {
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
pub struct I32Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i32, tokio_postgres::Error>,
    mapper: fn(i32) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I32Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i32) -> R) -> I32Query<'c, 'a, 's, C, R, N> {
        I32Query {
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
pub struct OptionStringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<Option<&str>, tokio_postgres::Error>,
    mapper: fn(Option<&str>) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> OptionStringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(Option<&str>) -> R) -> OptionStringQuery<'c, 'a, 's, C, R, N> {
        OptionStringQuery {
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
pub struct GetAllSongsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_all_songs() -> GetAllSongsStmt {
    GetAllSongsStmt(
        "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path FROM songs ORDER BY track_number",
        None,
    )
}
impl GetAllSongsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> SongQuery<'c, 'a, 's, C, Song, 0> {
        SongQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<SongBorrowed, tokio_postgres::Error> {
                Ok(SongBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    duration: row.try_get(2)?,
                    artist_id: row.try_get(3)?,
                    album_id: row.try_get(4)?,
                    release_date: row.try_get(5)?,
                    track_number: row.try_get(6)?,
                    image_path: row.try_get(7)?,
                })
            },
            mapper: |it| Song::from(it),
        }
    }
}
pub struct GetSongByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_song_by_id() -> GetSongByIdStmt {
    GetSongByIdStmt(
        "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path FROM songs WHERE id = $1",
        None,
    )
}
impl GetSongByIdStmt {
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
        id: &'a T1,
    ) -> SongQuery<'c, 'a, 's, C, Song, 1> {
        SongQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<SongBorrowed, tokio_postgres::Error> {
                Ok(SongBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    duration: row.try_get(2)?,
                    artist_id: row.try_get(3)?,
                    album_id: row.try_get(4)?,
                    release_date: row.try_get(5)?,
                    track_number: row.try_get(6)?,
                    image_path: row.try_get(7)?,
                })
            },
            mapper: |it| Song::from(it),
        }
    }
}
pub struct GetSongsByArtistStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_songs_by_artist() -> GetSongsByArtistStmt {
    GetSongsByArtistStmt(
        "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path FROM songs WHERE artist_id = $1 ORDER BY track_number",
        None,
    )
}
impl GetSongsByArtistStmt {
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
        artist_id: &'a T1,
    ) -> SongQuery<'c, 'a, 's, C, Song, 1> {
        SongQuery {
            client,
            params: [artist_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<SongBorrowed, tokio_postgres::Error> {
                Ok(SongBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    duration: row.try_get(2)?,
                    artist_id: row.try_get(3)?,
                    album_id: row.try_get(4)?,
                    release_date: row.try_get(5)?,
                    track_number: row.try_get(6)?,
                    image_path: row.try_get(7)?,
                })
            },
            mapper: |it| Song::from(it),
        }
    }
}
pub struct GetSongsByAlbumStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_songs_by_album() -> GetSongsByAlbumStmt {
    GetSongsByAlbumStmt(
        "SELECT id, name, duration, artist_id, album_id, release_date, track_number, image_path FROM songs WHERE album_id = $1 ORDER BY track_number",
        None,
    )
}
impl GetSongsByAlbumStmt {
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
        album_id: &'a T1,
    ) -> SongQuery<'c, 'a, 's, C, Song, 1> {
        SongQuery {
            client,
            params: [album_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<SongBorrowed, tokio_postgres::Error> {
                Ok(SongBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    duration: row.try_get(2)?,
                    artist_id: row.try_get(3)?,
                    album_id: row.try_get(4)?,
                    release_date: row.try_get(5)?,
                    track_number: row.try_get(6)?,
                    image_path: row.try_get(7)?,
                })
            },
            mapper: |it| Song::from(it),
        }
    }
}
pub struct CheckSongByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn check_song_by_id() -> CheckSongByIdStmt {
    CheckSongByIdStmt("SELECT 1 FROM songs WHERE id = $1", None)
}
impl CheckSongByIdStmt {
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
        id: &'a T1,
    ) -> I32Query<'c, 'a, 's, C, i32, 1> {
        I32Query {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct GetSongAlbumIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_song_album_id() -> GetSongAlbumIdStmt {
    GetSongAlbumIdStmt("SELECT album_id FROM songs WHERE id = $1", None)
}
impl GetSongAlbumIdStmt {
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
        id: &'a T1,
    ) -> OptionStringQuery<'c, 'a, 's, C, Option<String>, 1> {
        OptionStringQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.map(|v| v.into()),
        }
    }
}
pub struct InsertSongStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_song() -> InsertSongStmt {
    InsertSongStmt(
        "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        None,
    )
}
impl InsertSongStmt {
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
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        id: &'a T1,
        name: &'a T2,
        duration: &'a i32,
        artist_id: &'a T3,
        album_id: &'a Option<T4>,
        release_date: &'a Option<i64>,
        track_number: &'a Option<i32>,
        image_path: &'a Option<T5>,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(
                self.0,
                &[
                    id,
                    name,
                    duration,
                    artist_id,
                    album_id,
                    release_date,
                    track_number,
                    image_path,
                ],
            )
            .await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertSongParams<T1, T2, T3, T4, T5>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertSongStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a InsertSongParams<T1, T2, T3, T4, T5>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.id,
            &params.name,
            &params.duration,
            &params.artist_id,
            &params.album_id,
            &params.release_date,
            &params.track_number,
            &params.image_path,
        ))
    }
}
pub struct UpdateSongNameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_song_name() -> UpdateSongNameStmt {
    UpdateSongNameStmt("UPDATE songs SET name = $1 WHERE id = $2", None)
}
impl UpdateSongNameStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        name: &'a T1,
        id: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[name, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateSongNameParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateSongNameStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateSongNameParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.name, &params.id))
    }
}
pub struct UpdateSongDurationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_song_duration() -> UpdateSongDurationStmt {
    UpdateSongDurationStmt("UPDATE songs SET duration = $1 WHERE id = $2", None)
}
impl UpdateSongDurationStmt {
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
        duration: &'a i32,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[duration, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateSongDurationParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateSongDurationStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateSongDurationParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.duration, &params.id))
    }
}
pub struct UpdateSongReleaseDateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_song_release_date() -> UpdateSongReleaseDateStmt {
    UpdateSongReleaseDateStmt("UPDATE songs SET release_date = $1 WHERE id = $2", None)
}
impl UpdateSongReleaseDateStmt {
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
        release_date: &'a i64,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[release_date, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateSongReleaseDateParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateSongReleaseDateStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateSongReleaseDateParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.release_date, &params.id))
    }
}
pub struct UpdateSongTrackNumberStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_song_track_number() -> UpdateSongTrackNumberStmt {
    UpdateSongTrackNumberStmt("UPDATE songs SET track_number = $1 WHERE id = $2", None)
}
impl UpdateSongTrackNumberStmt {
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
        track_number: &'a i32,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[track_number, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateSongTrackNumberParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateSongTrackNumberStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateSongTrackNumberParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.track_number, &params.id))
    }
}
pub struct UpdateSongPartialStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_song_partial() -> UpdateSongPartialStmt {
    UpdateSongPartialStmt(
        "UPDATE songs SET name = COALESCE($1, name), duration = COALESCE($2, duration), artist_id = COALESCE($3, artist_id), album_id = COALESCE($4, album_id), release_date = COALESCE($5, release_date), track_number = COALESCE($6, track_number), image_path = COALESCE($7, image_path) WHERE id = $8",
        None,
    )
}
impl UpdateSongPartialStmt {
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
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        name: &'a Option<T1>,
        duration: &'a Option<i32>,
        artist_id: &'a Option<T2>,
        album_id: &'a Option<T3>,
        release_date: &'a Option<i64>,
        track_number: &'a Option<i32>,
        image_path: &'a Option<T4>,
        id: &'a T5,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(
                self.0,
                &[
                    name,
                    duration,
                    artist_id,
                    album_id,
                    release_date,
                    track_number,
                    image_path,
                    id,
                ],
            )
            .await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateSongPartialParams<T1, T2, T3, T4, T5>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateSongPartialStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateSongPartialParams<T1, T2, T3, T4, T5>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.name,
            &params.duration,
            &params.artist_id,
            &params.album_id,
            &params.release_date,
            &params.track_number,
            &params.image_path,
            &params.id,
        ))
    }
}
pub struct DeleteSongStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_song() -> DeleteSongStmt {
    DeleteSongStmt("DELETE FROM songs WHERE id = $1", None)
}
impl DeleteSongStmt {
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
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id]).await
    }
}
