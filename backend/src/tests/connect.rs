#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic, clippy::print_stderr)]

    use axum::Router;
    use connectrpc::client::{ClientConfig, HttpClient};
    use tokio::net::TcpListener;

    use crate::rpc::novasound::{album::v1 as album_v1, artist::v1, song::v1 as song_v1};
    use crate::services::connect::create_connect_router;
    use crate::tests::test_helpers::{TestSetupError, create_test_pool};

    async fn spawn_connect_test_server() -> Option<axum::http::Uri> {
        let pool = match create_test_pool().await {
            | Ok(pool) => pool,
            | Err(TestSetupError::MissingDatabaseUrl) => {
                assert!(
                    std::env::var_os("CI").is_none(),
                    "TEST_DATABASE_URL or DATABASE_URL must be set in CI"
                );
                eprintln!("Skipping test - TEST_DATABASE_URL or DATABASE_URL not set");
                return None;
            },
            | Err(TestSetupError::SetupFailed(error)) => {
                panic!("Test database setup failed: {error}");
            },
        };
        let app = Router::new().fallback_service(create_connect_router(pool).into_axum_service());
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind listener");
        let address = listener.local_addr().expect("listener addr");

        tokio::spawn(async move {
            axum::serve(listener, app).await.expect("serve app");
        });

        Some(format!("http://{address}").parse().expect("parse uri"))
    }

    #[tokio::test]
    async fn artist_connect_crud_round_trip() {
        let Some(uri) = spawn_connect_test_server().await else {
            return;
        };
        let client = v1::ArtistServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri));

        let created = client
            .create_artist(v1::CreateArtistRequest {
                name: "Connect Artist".to_string(),
                ..Default::default()
            })
            .await
            .expect("create artist")
            .into_owned();
        let artist = created
            .artist
            .into_option()
            .expect("created artist payload");

        let fetched = client
            .get_artist(v1::GetArtistRequest {
                id: artist.id.clone(),
                ..Default::default()
            })
            .await
            .expect("get artist")
            .into_owned();
        assert_eq!(fetched.name, "Connect Artist");

        let listed = client
            .get_artists(v1::GetArtistsRequest::default())
            .await
            .expect("get artists")
            .into_owned();
        assert_eq!(listed.artists.len(), 1);

        let updated = client
            .update_artist(v1::UpdateArtistRequest {
                id: artist.id.clone(),
                name: "Updated Connect Artist".to_string(),
                ..Default::default()
            })
            .await
            .expect("update artist")
            .into_owned();
        assert_eq!(
            updated.artist.into_option().expect("updated artist").name,
            "Updated Connect Artist"
        );

        client
            .delete_artist(v1::DeleteArtistRequest {
                id: artist.id.clone(),
                ..Default::default()
            })
            .await
            .expect("delete artist");
        let deleted_error = client
            .get_artist(v1::GetArtistRequest {
                id: artist.id,
                ..Default::default()
            })
            .await
            .expect_err("deleted artist should be missing");
        assert_eq!(deleted_error.code, connectrpc::ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn album_connect_crud_round_trip() {
        let Some(uri) = spawn_connect_test_server().await else {
            return;
        };
        let artist_client =
            v1::ArtistServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri.clone()));
        let album_client =
            album_v1::AlbumServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri));

        let artist = artist_client
            .create_artist(v1::CreateArtistRequest {
                name: "Album Artist".to_string(),
                ..Default::default()
            })
            .await
            .expect("create artist")
            .into_owned()
            .artist
            .into_option()
            .expect("artist payload");

        let created = album_client
            .create_album(album_v1::CreateAlbumRequest {
                name: "Connect Album".to_string(),
                release_date: Some("15-06-2024".to_string()),
                artist_id: artist.id,
                album_type: Some(album_v1::AlbumType::ALBUM_TYPE_EP.into()),
                ..Default::default()
            })
            .await
            .expect("create album")
            .into_owned();
        let album = created.album.into_option().expect("album payload");

        let listed = album_client
            .get_albums(album_v1::GetAlbumsRequest::default())
            .await
            .expect("get albums")
            .into_owned();
        assert_eq!(listed.albums.len(), 1);

        let fetched = album_client
            .get_album(album_v1::GetAlbumRequest {
                id: album.id.clone(),
                ..Default::default()
            })
            .await
            .expect("get album")
            .into_owned();
        assert_eq!(fetched.release_date.as_deref(), Some("15-06-2024"));

        let updated = album_client
            .update_album(album_v1::UpdateAlbumRequest {
                id: album.id.clone(),
                name: Some("Updated Connect Album".to_string()),
                release_date: Some("16-06-2024".to_string()),
                artist_id: None,
                ..Default::default()
            })
            .await
            .expect("update album")
            .into_owned();
        let updated_album = updated.album.into_option().expect("updated album");
        assert_eq!(updated_album.name, "Updated Connect Album");
        assert_eq!(updated_album.release_date.as_deref(), Some("16-06-2024"));

        album_client
            .delete_album(album_v1::DeleteAlbumRequest {
                id: album.id.clone(),
                ..Default::default()
            })
            .await
            .expect("delete album");
        let deleted_error = album_client
            .get_album(album_v1::GetAlbumRequest {
                id: album.id,
                ..Default::default()
            })
            .await
            .expect_err("deleted album should be missing");
        assert_eq!(deleted_error.code, connectrpc::ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn song_connect_crud_round_trip() {
        let Some(uri) = spawn_connect_test_server().await else {
            return;
        };
        let artist_client =
            v1::ArtistServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri.clone()));
        let song_client =
            song_v1::SongServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri));

        let artist = artist_client
            .create_artist(v1::CreateArtistRequest {
                name: "Song Artist".to_string(),
                ..Default::default()
            })
            .await
            .expect("create artist")
            .into_owned()
            .artist
            .into_option()
            .expect("artist payload");

        let created = song_client
            .create_song(song_v1::CreateSongRequest {
                name: "Connect Song".to_string(),
                duration: 240,
                artist_id: artist.id,
                album_id: None,
                release_date: Some("01-01-2024".to_string()),
                track_number: Some(1),
                ..Default::default()
            })
            .await
            .expect("create song")
            .into_owned();
        let song = created.song.into_option().expect("song payload");

        let listed = song_client
            .get_songs(song_v1::GetSongsRequest::default())
            .await
            .expect("get songs")
            .into_owned();
        assert_eq!(listed.songs.len(), 1);

        let fetched = song_client
            .get_song(song_v1::GetSongRequest {
                id: song.id.clone(),
                ..Default::default()
            })
            .await
            .expect("get song")
            .into_owned();
        assert_eq!(fetched.duration, 240);

        let updated = song_client
            .update_song(song_v1::UpdateSongRequest {
                id: song.id.clone(),
                name: Some("Updated Connect Song".to_string()),
                duration: Some(245),
                release_date: Some("02-01-2024".to_string()),
                track_number: Some(2),
                ..Default::default()
            })
            .await
            .expect("update song")
            .into_owned();
        let updated_song = updated.song.into_option().expect("updated song");
        assert_eq!(updated_song.name, "Updated Connect Song");
        assert_eq!(updated_song.duration, 245);

        song_client
            .delete_song(song_v1::DeleteSongRequest {
                id: song.id.clone(),
                ..Default::default()
            })
            .await
            .expect("delete song");
        let deleted_error = song_client
            .get_song(song_v1::GetSongRequest {
                id: song.id,
                ..Default::default()
            })
            .await
            .expect_err("deleted song should be missing");
        assert_eq!(deleted_error.code, connectrpc::ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn connect_maps_not_found_and_invalid_argument_errors() {
        let Some(uri) = spawn_connect_test_server().await else {
            return;
        };
        let artist_client =
            v1::ArtistServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri.clone()));
        let album_client =
            album_v1::AlbumServiceClient::new(HttpClient::plaintext(), ClientConfig::new(uri));

        let missing_error = artist_client
            .get_artist(v1::GetArtistRequest {
                id: "missing-id".to_string(),
                ..Default::default()
            })
            .await
            .expect_err("missing artist should fail");
        assert_eq!(missing_error.code, connectrpc::ErrorCode::NotFound);

        let artist = artist_client
            .create_artist(v1::CreateArtistRequest {
                name: "Date Artist".to_string(),
                ..Default::default()
            })
            .await
            .expect("create artist")
            .into_owned()
            .artist
            .into_option()
            .expect("artist payload");

        let invalid_error = album_client
            .create_album(album_v1::CreateAlbumRequest {
                name: "Broken Date".to_string(),
                release_date: Some("2024-06-15".to_string()),
                artist_id: artist.id,
                album_type: None,
                ..Default::default()
            })
            .await
            .expect_err("invalid date should fail");
        assert_eq!(invalid_error.code, connectrpc::ErrorCode::InvalidArgument);
    }
}
