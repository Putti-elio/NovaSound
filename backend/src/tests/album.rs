#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::too_many_arguments)]
    use chrono::NaiveDate;
    use deadpool_postgres::{Config, Pool, Runtime};
    use tokio_postgres::NoTls;

    use crate::migrations::{apply_migrations, reset_database};
    use crate::models::album_model::{CreateAlbum, UpdateAlbum};
    use crate::models::song_model::AlbumType;
    use crate::services::album_service;

    async fn create_test_pool() -> Pool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .or_else(|_| std::env::var("DATABASE_URL"))
            .expect("TEST_DATABASE_URL or DATABASE_URL must be set");

        let mut cfg = Config::new();
        cfg.url = Some(database_url);

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Failed to create test pool");

        let mut client = pool.get().await.expect("Failed to get test client");

        reset_database(&client)
            .await
            .expect("Failed to reset test schema");

        apply_migrations(&mut client)
            .await
            .expect("Failed to apply test migrations");

        pool
    }

    async fn insert_artist(pool: &Pool, artist_id: &str, name: &str) {
        let client = pool.get().await.expect("Failed to get client");
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&artist_id, &name, &format!("/images/{}", name)],
            )
            .await
            .expect("Insert artist failed");
    }

    async fn insert_album(
        pool: &Pool,
        album_id: &str,
        name: &str,
        total_duration: i32,
        artist_id: &str,
        image_path: &str,
        album_type: &str,
    ) {
        let client = pool.get().await.expect("Failed to get client");
        client
            .execute(
                "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES ($1, $2, $3, $4, $5, $6)",
                &[&album_id, &name, &total_duration, &artist_id, &image_path, &album_type],
            )
            .await
            .expect("Insert album failed");
    }

    // ==================== CREATE ====================

    #[tokio::test]
    async fn test_create_album_success() {
        let pool = create_test_pool().await;
        let artist_id = "artist-001";
        insert_artist(&pool, artist_id, "Test Artist").await;

        let album = CreateAlbum {
            name: "Test Album".to_string(),
            release_date: NaiveDate::from_ymd_opt(2024, 6, 15),
            artist_id: artist_id.to_string(),
            album_type: Some(AlbumType::Album),
        };

        let result = album_service::create_album(&pool, album).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_album_empty_name() {
        let pool = create_test_pool().await;
        let artist_id = "artist-002";
        insert_artist(&pool, artist_id, "Test Artist").await;

        let album = CreateAlbum {
            name: String::new(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&pool, album).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_album_whitespace_name() {
        let pool = create_test_pool().await;
        let artist_id = "artist-003";
        insert_artist(&pool, artist_id, "Test Artist").await;

        let album = CreateAlbum {
            name: " ".to_string(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&pool, album).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_album_invalid_artist() {
        let pool = create_test_pool().await;

        let album = CreateAlbum {
            name: "Test Album".to_string(),
            release_date: None,
            artist_id: "nonexistent-artist".to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&pool, album).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_album_duplicate_for_artist() {
        let pool = create_test_pool().await;
        let artist_id = "artist-004";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            "existing-album",
            "Duplicate Album",
            0,
            artist_id,
            "/images/existing",
            "ALBUM",
        )
        .await;

        let album = CreateAlbum {
            name: "Duplicate Album".to_string(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&pool, album).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_album_same_name_different_artist() {
        let pool = create_test_pool().await;
        let artist1_id = "artist-005a";
        let artist2_id = "artist-005b";
        insert_artist(&pool, artist1_id, "Artist One").await;
        insert_artist(&pool, artist2_id, "Artist Two").await;
        insert_album(
            &pool,
            "album-a",
            "Shared Name",
            0,
            artist1_id,
            "/images/a",
            "ALBUM",
        )
        .await;

        let album = CreateAlbum {
            name: "Shared Name".to_string(),
            release_date: None,
            artist_id: artist2_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&pool, album).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_album_default_type() {
        let pool = create_test_pool().await;
        let artist_id = "artist-006";
        insert_artist(&pool, artist_id, "Test Artist").await;

        let album = CreateAlbum {
            name: "Default Type Album".to_string(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        album_service::create_album(&pool, album).await.unwrap();

        let client = pool.get().await.expect("Failed to get client");
        let album_type: String = client
            .query_one(
                "SELECT album_type FROM albums WHERE name = $1",
                &[&"Default Type Album"],
            )
            .await
            .expect("Query failed")
            .get(0);
        assert_eq!(album_type, "ALBUM");
    }

    // ==================== GET ALL ====================

    #[tokio::test]
    async fn test_get_all_albums_empty() {
        let pool = create_test_pool().await;

        let result = album_service::get_all_albums(&pool).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_all_albums_with_data() {
        let pool = create_test_pool().await;
        let artist_id = "artist-007";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            "album-1",
            "Album One",
            1200,
            artist_id,
            "/images/one",
            "ALBUM",
        )
        .await;
        insert_album(
            &pool,
            "album-2",
            "Album Two",
            600,
            artist_id,
            "/images/two",
            "EP",
        )
        .await;
        insert_album(
            &pool,
            "album-3",
            "Album Three",
            180,
            artist_id,
            "/images/three",
            "SINGLE",
        )
        .await;

        let result = album_service::get_all_albums(&pool).await;
        assert!(result.is_ok());
        let albums = result.unwrap();
        assert_eq!(albums.len(), 3);

        let names: Vec<String> = albums.iter().map(|a| a.name.clone()).collect();
        assert!(names.contains(&"Album One".to_string()));
        assert!(names.contains(&"Album Two".to_string()));
        assert!(names.contains(&"Album Three".to_string()));
    }

    // ==================== GET BY ID ====================

    #[tokio::test]
    async fn test_get_album_by_id_not_found() {
        let pool = create_test_pool().await;

        let result = album_service::get_album_by_id(&pool, &uuid::Uuid::new_v4().to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_album_by_id_success() {
        let pool = create_test_pool().await;
        let artist_id = "artist-008";
        insert_artist(&pool, artist_id, "Test Artist").await;
        let expected_id = "album-find-me";
        insert_album(
            &pool,
            expected_id,
            "Find Me Album",
            900,
            artist_id,
            "/images/find_me",
            "EP",
        )
        .await;

        let result = album_service::get_album_by_id(&pool, expected_id).await;
        assert!(result.is_ok());
        let album = result.unwrap();
        assert_eq!(album.id, expected_id);
        assert_eq!(album.name, "Find Me Album");
        assert_eq!(album.total_duration, 900);
        assert_eq!(album.artist_id, artist_id);
        assert_eq!(album.album_type, AlbumType::Ep);
    }

    // ==================== GET BY ARTIST ====================

    #[tokio::test]
    async fn test_get_albums_by_artist() {
        let pool = create_test_pool().await;
        let artist1_id = "artist-009";
        let artist2_id = "artist-010";
        insert_artist(&pool, artist1_id, "Artist One").await;
        insert_artist(&pool, artist2_id, "Artist Two").await;
        insert_album(
            &pool,
            "album-a1",
            "Artist 1 Album 1",
            1200,
            artist1_id,
            "/images/a1b1",
            "ALBUM",
        )
        .await;
        insert_album(
            &pool,
            "album-a2",
            "Artist 1 Album 2",
            600,
            artist1_id,
            "/images/a1b2",
            "EP",
        )
        .await;
        insert_album(
            &pool,
            "album-a3",
            "Artist 1 Album 3",
            180,
            artist1_id,
            "/images/a1b3",
            "SINGLE",
        )
        .await;
        insert_album(
            &pool,
            "album-b1",
            "Artist 2 Album 1",
            1500,
            artist2_id,
            "/images/a2b1",
            "ALBUM",
        )
        .await;
        insert_album(
            &pool,
            "album-b2",
            "Artist 2 Album 2",
            300,
            artist2_id,
            "/images/a2b2",
            "SINGLE",
        )
        .await;

        let albums = album_service::get_albums_by_artist(&pool, artist1_id)
            .await
            .unwrap();
        assert_eq!(albums.len(), 3);

        let albums = album_service::get_albums_by_artist(&pool, artist2_id)
            .await
            .unwrap();
        assert_eq!(albums.len(), 2);
    }

    // ==================== UPDATE ====================

    #[tokio::test]
    async fn test_update_album_name() {
        let pool = create_test_pool().await;
        let artist_id = "artist-011";
        let album_id = "album-update-name";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Old Name",
            0,
            artist_id,
            "/images/old",
            "ALBUM",
        )
        .await;

        let update = UpdateAlbum {
            name: Some("Updated Name".to_string()),
            release_date: None,
            artist_id: None,
        };

        let result = album_service::update_album(&pool, album_id, update).await;
        assert!(result.is_ok());

        let client = pool.get().await.expect("Failed to get client");
        let name: String = client
            .query_one("SELECT name FROM albums WHERE id = $1", &[&album_id])
            .await
            .expect("Query failed")
            .get(0);
        assert_eq!(name, "Updated Name");
    }

    #[tokio::test]
    async fn test_update_album_empty_name() {
        let pool = create_test_pool().await;
        let artist_id = "artist-012";
        let album_id = "album-empty-name";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Valid Name",
            0,
            artist_id,
            "/images/valid",
            "ALBUM",
        )
        .await;

        let update = UpdateAlbum {
            name: Some(String::new()),
            release_date: None,
            artist_id: None,
        };

        let result = album_service::update_album(&pool, album_id, update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_album_artist() {
        let pool = create_test_pool().await;
        let artist1_id = "artist-013a";
        let artist2_id = "artist-013b";
        let album_id = "album-update-artist";
        insert_artist(&pool, artist1_id, "Artist One").await;
        insert_artist(&pool, artist2_id, "Artist Two").await;
        insert_album(
            &pool,
            album_id,
            "Transfer Album",
            0,
            artist1_id,
            "/images/transfer",
            "ALBUM",
        )
        .await;

        let update = UpdateAlbum {
            name: None,
            release_date: None,
            artist_id: Some(artist2_id.to_string()),
        };

        let result = album_service::update_album(&pool, album_id, update).await;
        assert!(result.is_ok());

        let client = pool.get().await.expect("Failed to get client");
        let artist_id_result: String = client
            .query_one("SELECT artist_id FROM albums WHERE id = $1", &[&album_id])
            .await
            .expect("Query failed")
            .get(0);
        assert_eq!(artist_id_result, artist2_id);
    }

    #[tokio::test]
    async fn test_update_album_invalid_artist() {
        let pool = create_test_pool().await;
        let artist_id = "artist-014";
        let album_id = "album-invalid-artist";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Test Album",
            0,
            artist_id,
            "/images/test",
            "ALBUM",
        )
        .await;

        let update = UpdateAlbum {
            name: None,
            release_date: None,
            artist_id: Some("nonexistent-artist".to_string()),
        };

        let result = album_service::update_album(&pool, album_id, update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_album_not_found() {
        let pool = create_test_pool().await;

        let update = UpdateAlbum {
            name: Some("New Name".to_string()),
            release_date: None,
            artist_id: None,
        };

        let result =
            album_service::update_album(&pool, &uuid::Uuid::new_v4().to_string(), update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_album_no_changes() {
        let pool = create_test_pool().await;
        let artist_id = "artist-015";
        let album_id = "album-no-changes";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Unchanged",
            500,
            artist_id,
            "/images/unchanged",
            "ALBUM",
        )
        .await;

        let update = UpdateAlbum {
            name: None,
            release_date: None,
            artist_id: None,
        };

        let result = album_service::update_album(&pool, album_id, update).await;
        assert!(result.is_ok());
    }

    // ==================== DELETE ====================

    #[tokio::test]
    async fn test_delete_album_success() {
        let pool = create_test_pool().await;
        let artist_id = "artist-016";
        let album_id = "album-delete";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "To Delete",
            0,
            artist_id,
            "/images/delete",
            "ALBUM",
        )
        .await;

        let result = album_service::delete_album(&pool, album_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_album_not_found() {
        let pool = create_test_pool().await;

        let result = album_service::delete_album(&pool, &uuid::Uuid::new_v4().to_string()).await;
        assert!(result.is_err());
    }

    // ==================== UPDATE DURATION ====================

    #[tokio::test]
    async fn test_update_album_duration() {
        let pool = create_test_pool().await;
        let artist_id = "artist-017";
        let album_id = "album-duration";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Duration Album",
            0,
            artist_id,
            "/images/duration",
            "ALBUM",
        )
        .await;

        let client = pool.get().await.expect("Failed to get client");
        client
            .execute(
                "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&"song-d1", &"Song 1", &200i32, &artist_id, &album_id, &None::<i64>, &1i32, &"/images/d1"],
            )
            .await
            .expect("Insert failed");
        client
            .execute(
                "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&"song-d2", &"Song 2", &300i32, &artist_id, &album_id, &None::<i64>, &2i32, &"/images/d2"],
            )
            .await
            .expect("Insert failed");
        client
            .execute(
                "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&"song-d3", &"Song 3", &250i32, &artist_id, &album_id, &None::<i64>, &3i32, &"/images/d3"],
            )
            .await
            .expect("Insert failed");

        let result = album_service::update_album_duration(&pool, album_id).await;
        assert!(result.is_ok());

        let total_duration: i32 = client
            .query_one(
                "SELECT total_duration FROM albums WHERE id = $1",
                &[&album_id],
            )
            .await
            .expect("Query failed")
            .get(0);
        assert_eq!(total_duration, 750);
    }
}
