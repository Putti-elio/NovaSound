// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CheckAlbumByNameAndArtistParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub artist_id: T2,
}
#[derive(Debug)]
pub struct GetStandaloneCollectionIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub artist_id: T1,
    pub album_type: T2,
}
#[derive(Debug)]
pub struct InsertAlbumParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub id: T1,
    pub name: T2,
    pub release_date: Option<i64>,
    pub artist_id: T3,
    pub image_path: T4,
    pub album_type: T5,
}
#[derive(Debug)]
pub struct InsertStandaloneCollectionParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub id: T1,
    pub name: T2,
    pub artist_id: T3,
    pub image_path: T4,
    pub album_type: T5,
}
#[derive(Debug)]
pub struct UpdateAlbumNameParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub id: T2,
}
#[derive(Debug)]
pub struct UpdateAlbumReleaseDateParams<T1: crate::StringSql> {
    pub release_date: i64,
    pub id: T1,
}
#[derive(Debug)]
pub struct UpdateAlbumArtistIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub artist_id: T1,
    pub id: T2,
}
#[derive(Debug)]
pub struct UpdateAlbumDurationParams<T1: crate::StringSql> {
    pub total_duration: i32,
    pub id: T1,
}
#[derive(Debug)]
pub struct UpdateAlbumDurationAndTypeParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub total_duration: i32,
    pub album_type: T1,
    pub id: T2,
}
#[derive(Debug)]
pub struct UpdateAlbumPartialParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub name: Option<T1>,
    pub release_date: Option<i64>,
    pub artist_id: Option<T2>,
    pub total_duration: Option<i32>,
    pub image_path: Option<T3>,
    pub album_type: Option<T4>,
    pub id: T5,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub total_duration: i32,
    pub release_date: Option<i64>,
    pub artist_id: String,
    pub image_path: Option<String>,
    pub album_type: String,
}
pub struct AlbumBorrowed<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub total_duration: i32,
    pub release_date: Option<i64>,
    pub artist_id: &'a str,
    pub image_path: Option<&'a str>,
    pub album_type: &'a str,
}
impl<'a> From<AlbumBorrowed<'a>> for Album {
    fn from(
        AlbumBorrowed {
            id,
            name,
            total_duration,
            release_date,
            artist_id,
            image_path,
            album_type,
        }: AlbumBorrowed<'a>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            total_duration,
            release_date,
            artist_id: artist_id.into(),
            image_path: image_path.map(|v| v.into()),
            album_type: album_type.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GetAlbumSongStats {
    pub song_count: i64,
    pub total_duration: i64,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct AlbumQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<AlbumBorrowed, tokio_postgres::Error>,
    mapper: fn(AlbumBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AlbumQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(AlbumBorrowed) -> R) -> AlbumQuery<'c, 'a, 's, C, R, N> {
        AlbumQuery {
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
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
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
pub struct GetAlbumSongStatsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetAlbumSongStats, tokio_postgres::Error>,
    mapper: fn(GetAlbumSongStats) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetAlbumSongStatsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetAlbumSongStats) -> R,
    ) -> GetAlbumSongStatsQuery<'c, 'a, 's, C, R, N> {
        GetAlbumSongStatsQuery {
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
pub struct GetAllAlbumsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_all_albums() -> GetAllAlbumsStmt {
    GetAllAlbumsStmt(
        "SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums",
        None,
    )
}
impl GetAllAlbumsStmt {
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
    ) -> AlbumQuery<'c, 'a, 's, C, Album, 0> {
        AlbumQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<AlbumBorrowed, tokio_postgres::Error> {
                Ok(AlbumBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    total_duration: row.try_get(2)?,
                    release_date: row.try_get(3)?,
                    artist_id: row.try_get(4)?,
                    image_path: row.try_get(5)?,
                    album_type: row.try_get(6)?,
                })
            },
            mapper: |it| Album::from(it),
        }
    }
}
pub struct GetAlbumByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_album_by_id() -> GetAlbumByIdStmt {
    GetAlbumByIdStmt(
        "SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE id = $1",
        None,
    )
}
impl GetAlbumByIdStmt {
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
    ) -> AlbumQuery<'c, 'a, 's, C, Album, 1> {
        AlbumQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<AlbumBorrowed, tokio_postgres::Error> {
                Ok(AlbumBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    total_duration: row.try_get(2)?,
                    release_date: row.try_get(3)?,
                    artist_id: row.try_get(4)?,
                    image_path: row.try_get(5)?,
                    album_type: row.try_get(6)?,
                })
            },
            mapper: |it| Album::from(it),
        }
    }
}
pub struct GetAlbumsByArtistStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_albums_by_artist() -> GetAlbumsByArtistStmt {
    GetAlbumsByArtistStmt(
        "SELECT id, name, total_duration, release_date, artist_id, image_path, album_type FROM albums WHERE artist_id = $1",
        None,
    )
}
impl GetAlbumsByArtistStmt {
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
    ) -> AlbumQuery<'c, 'a, 's, C, Album, 1> {
        AlbumQuery {
            client,
            params: [artist_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<AlbumBorrowed, tokio_postgres::Error> {
                Ok(AlbumBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    total_duration: row.try_get(2)?,
                    release_date: row.try_get(3)?,
                    artist_id: row.try_get(4)?,
                    image_path: row.try_get(5)?,
                    album_type: row.try_get(6)?,
                })
            },
            mapper: |it| Album::from(it),
        }
    }
}
pub struct CheckAlbumByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn check_album_by_id() -> CheckAlbumByIdStmt {
    CheckAlbumByIdStmt("SELECT 1 FROM albums WHERE id = $1", None)
}
impl CheckAlbumByIdStmt {
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
pub struct CheckAlbumByNameAndArtistStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn check_album_by_name_and_artist() -> CheckAlbumByNameAndArtistStmt {
    CheckAlbumByNameAndArtistStmt(
        "SELECT 1 FROM albums WHERE name = $1 AND artist_id = $2",
        None,
    )
}
impl CheckAlbumByNameAndArtistStmt {
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
        name: &'a T1,
        artist_id: &'a T2,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        I32Query {
            client,
            params: [name, artist_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CheckAlbumByNameAndArtistParams<T1, T2>,
        I32Query<'c, 'a, 's, C, i32, 2>,
        C,
    > for CheckAlbumByNameAndArtistStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CheckAlbumByNameAndArtistParams<T1, T2>,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        self.bind(client, &params.name, &params.artist_id)
    }
}
pub struct CheckAlbumByIdSimpleStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn check_album_by_id_simple() -> CheckAlbumByIdSimpleStmt {
    CheckAlbumByIdSimpleStmt("SELECT 1 FROM albums WHERE id = $1", None)
}
impl CheckAlbumByIdSimpleStmt {
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
pub struct GetAlbumImagePathStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_album_image_path() -> GetAlbumImagePathStmt {
    GetAlbumImagePathStmt("SELECT image_path FROM albums WHERE id = $1", None)
}
impl GetAlbumImagePathStmt {
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
pub struct GetAlbumTypeByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_album_type_by_id() -> GetAlbumTypeByIdStmt {
    GetAlbumTypeByIdStmt("SELECT album_type FROM albums WHERE id = $1", None)
}
impl GetAlbumTypeByIdStmt {
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
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct GetStandaloneCollectionIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_standalone_collection_id() -> GetStandaloneCollectionIdStmt {
    GetStandaloneCollectionIdStmt(
        "SELECT id FROM albums WHERE artist_id = $1 AND album_type = $2",
        None,
    )
}
impl GetStandaloneCollectionIdStmt {
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
        artist_id: &'a T1,
        album_type: &'a T2,
    ) -> OptionStringQuery<'c, 'a, 's, C, Option<String>, 2> {
        OptionStringQuery {
            client,
            params: [artist_id, album_type],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.map(|v| v.into()),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetStandaloneCollectionIdParams<T1, T2>,
        OptionStringQuery<'c, 'a, 's, C, Option<String>, 2>,
        C,
    > for GetStandaloneCollectionIdStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetStandaloneCollectionIdParams<T1, T2>,
    ) -> OptionStringQuery<'c, 'a, 's, C, Option<String>, 2> {
        self.bind(client, &params.artist_id, &params.album_type)
    }
}
pub struct GetAlbumSongStatsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_album_song_stats() -> GetAlbumSongStatsStmt {
    GetAlbumSongStatsStmt(
        "SELECT COUNT(*) AS song_count, COALESCE(SUM(duration), 0) AS total_duration FROM songs WHERE album_id = $1",
        None,
    )
}
impl GetAlbumSongStatsStmt {
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
    ) -> GetAlbumSongStatsQuery<'c, 'a, 's, C, GetAlbumSongStats, 1> {
        GetAlbumSongStatsQuery {
            client,
            params: [album_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetAlbumSongStats, tokio_postgres::Error> {
                    Ok(GetAlbumSongStats {
                        song_count: row.try_get(0)?,
                        total_duration: row.try_get(1)?,
                    })
                },
            mapper: |it| GetAlbumSongStats::from(it),
        }
    }
}
pub struct InsertAlbumStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_album() -> InsertAlbumStmt {
    InsertAlbumStmt(
        "INSERT INTO albums (id, name, total_duration, release_date, artist_id, image_path, album_type) VALUES ($1, $2, 0, $3, $4, $5, $6)",
        None,
    )
}
impl InsertAlbumStmt {
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
        release_date: &'a Option<i64>,
        artist_id: &'a T3,
        image_path: &'a T4,
        album_type: &'a T5,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(
                self.0,
                &[id, name, release_date, artist_id, image_path, album_type],
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
        InsertAlbumParams<T1, T2, T3, T4, T5>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertAlbumStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a InsertAlbumParams<T1, T2, T3, T4, T5>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.id,
            &params.name,
            &params.release_date,
            &params.artist_id,
            &params.image_path,
            &params.album_type,
        ))
    }
}
pub struct InsertStandaloneCollectionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_standalone_collection() -> InsertStandaloneCollectionStmt {
    InsertStandaloneCollectionStmt(
        "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES ($1, $2, 0, $3, $4, $5)",
        None,
    )
}
impl InsertStandaloneCollectionStmt {
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
        artist_id: &'a T3,
        image_path: &'a T4,
        album_type: &'a T5,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[id, name, artist_id, image_path, album_type])
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
        InsertStandaloneCollectionParams<T1, T2, T3, T4, T5>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertStandaloneCollectionStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a InsertStandaloneCollectionParams<T1, T2, T3, T4, T5>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.id,
            &params.name,
            &params.artist_id,
            &params.image_path,
            &params.album_type,
        ))
    }
}
pub struct UpdateAlbumNameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_album_name() -> UpdateAlbumNameStmt {
    UpdateAlbumNameStmt("UPDATE albums SET name = $1 WHERE id = $2", None)
}
impl UpdateAlbumNameStmt {
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
        UpdateAlbumNameParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAlbumNameStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateAlbumNameParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.name, &params.id))
    }
}
pub struct UpdateAlbumReleaseDateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_album_release_date() -> UpdateAlbumReleaseDateStmt {
    UpdateAlbumReleaseDateStmt("UPDATE albums SET release_date = $1 WHERE id = $2", None)
}
impl UpdateAlbumReleaseDateStmt {
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
        UpdateAlbumReleaseDateParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAlbumReleaseDateStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateAlbumReleaseDateParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.release_date, &params.id))
    }
}
pub struct UpdateAlbumArtistIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_album_artist_id() -> UpdateAlbumArtistIdStmt {
    UpdateAlbumArtistIdStmt("UPDATE albums SET artist_id = $1 WHERE id = $2", None)
}
impl UpdateAlbumArtistIdStmt {
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
        artist_id: &'a T1,
        id: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[artist_id, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateAlbumArtistIdParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAlbumArtistIdStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateAlbumArtistIdParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.artist_id, &params.id))
    }
}
pub struct UpdateAlbumDurationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_album_duration() -> UpdateAlbumDurationStmt {
    UpdateAlbumDurationStmt("UPDATE albums SET total_duration = $1 WHERE id = $2", None)
}
impl UpdateAlbumDurationStmt {
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
        total_duration: &'a i32,
        id: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[total_duration, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateAlbumDurationParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAlbumDurationStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateAlbumDurationParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.total_duration, &params.id))
    }
}
pub struct UpdateAlbumDurationAndTypeStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_album_duration_and_type() -> UpdateAlbumDurationAndTypeStmt {
    UpdateAlbumDurationAndTypeStmt(
        "UPDATE albums SET total_duration = $1, album_type = $2 WHERE id = $3",
        None,
    )
}
impl UpdateAlbumDurationAndTypeStmt {
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
        total_duration: &'a i32,
        album_type: &'a T1,
        id: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[total_duration, album_type, id])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateAlbumDurationAndTypeParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAlbumDurationAndTypeStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateAlbumDurationAndTypeParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.total_duration,
            &params.album_type,
            &params.id,
        ))
    }
}
pub struct UpdateAlbumPartialStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_album_partial() -> UpdateAlbumPartialStmt {
    UpdateAlbumPartialStmt(
        "UPDATE albums SET name = COALESCE($1, name), release_date = COALESCE($2, release_date), artist_id = COALESCE($3, artist_id), total_duration = COALESCE($4, total_duration), image_path = COALESCE($5, image_path), album_type = COALESCE($6, album_type) WHERE id = $7",
        None,
    )
}
impl UpdateAlbumPartialStmt {
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
        release_date: &'a Option<i64>,
        artist_id: &'a Option<T2>,
        total_duration: &'a Option<i32>,
        image_path: &'a Option<T3>,
        album_type: &'a Option<T4>,
        id: &'a T5,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(
                self.0,
                &[
                    name,
                    release_date,
                    artist_id,
                    total_duration,
                    image_path,
                    album_type,
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
        UpdateAlbumPartialParams<T1, T2, T3, T4, T5>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAlbumPartialStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateAlbumPartialParams<T1, T2, T3, T4, T5>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.name,
            &params.release_date,
            &params.artist_id,
            &params.total_duration,
            &params.image_path,
            &params.album_type,
            &params.id,
        ))
    }
}
pub struct CalcAlbumDurationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn calc_album_duration() -> CalcAlbumDurationStmt {
    CalcAlbumDurationStmt(
        "SELECT COALESCE(SUM(duration), 0) AS total_duration FROM songs WHERE album_id = $1",
        None,
    )
}
impl CalcAlbumDurationStmt {
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
    ) -> I64Query<'c, 'a, 's, C, i64, 1> {
        I64Query {
            client,
            params: [album_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct DeleteAlbumStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_album() -> DeleteAlbumStmt {
    DeleteAlbumStmt("DELETE FROM albums WHERE id = $1", None)
}
impl DeleteAlbumStmt {
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
