#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::too_many_arguments)]
    use chrono::NaiveDate;
    use deadpool_postgres::{Config, Pool, Runtime};
    use tokio_postgres::NoTls;

    use crate::migrations::{apply_migrations, reset_database};
    use crate::models::song_model::{CreateSong, UpdateSong};
    use crate::services::song_service;

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

    async fn insert_song(
        pool: &Pool,
        song_id: &str,
        name: &str,
        duration: i32,
        artist_id: &str,
        album_id: Option<&str>,
        track_number: Option<i32>,
        image_path: &str,
    ) {
        let client = pool.get().await.expect("Failed to get client");
        client
            .execute(
                "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[&song_id, &name, &duration, &artist_id, &album_id, &None::<i64>, &track_number, &image_path],
            )
            .await
            .expect("Insert song failed");
    }

    // ==================== CREATE ====================

    #[tokio::test]
    async fn test_create_song_success_with_album() {
        let pool = create_test_pool().await;
        let artist_id = "artist-001";
        let album_id = "album-001";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Test Album",
            0,
            artist_id,
            "/images/Test_Album",
            "ALBUM",
        )
        .await;

        let song = CreateSong {
            name: "Test Song".to_string(),
            duration: 240,
            artist_id: artist_id.to_string(),
            album_id: Some(album_id.to_string()),
            release_date: NaiveDate::from_ymd_opt(2024, 1, 1),
            track_number: Some(1),
        };

        let result = song_service::create_song(&pool, song).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_song_success_without_album() {
        let pool = create_test_pool().await;
        let artist_id = "artist-002";
        insert_artist(&pool, artist_id, "Solo Artist").await;

        let song = CreateSong {
            name: "Standalone Song".to_string(),
            duration: 180,
            artist_id: artist_id.to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&pool, song).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_song_empty_name() {
        let pool = create_test_pool().await;
        let artist_id = "artist-003";
        insert_artist(&pool, artist_id, "Test Artist").await;

        let song = CreateSong {
            name: String::new(),
            duration: 240,
            artist_id: artist_id.to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&pool, song).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_song_invalid_artist() {
        let pool = create_test_pool().await;

        let song = CreateSong {
            name: "Test Song".to_string(),
            duration: 240,
            artist_id: "nonexistent-artist".to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&pool, song).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_song_invalid_album() {
        let pool = create_test_pool().await;
        let artist_id = "artist-004";
        insert_artist(&pool, artist_id, "Test Artist").await;

        let song = CreateSong {
            name: "Test Song".to_string(),
            duration: 240,
            artist_id: artist_id.to_string(),
            album_id: Some("nonexistent-album".to_string()),
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&pool, song).await;
        assert!(result.is_err());
    }

    // ==================== GET ALL ====================

    #[tokio::test]
    async fn test_get_all_songs_empty() {
        let pool = create_test_pool().await;

        let result = song_service::get_all_songs(&pool).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_all_songs_with_data() {
        let pool = create_test_pool().await;
        let artist_id = "artist-005";
        let album_id = "album-005";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Test Album",
            0,
            artist_id,
            "/images/Test_Album",
            "ALBUM",
        )
        .await;
        insert_song(
            &pool,
            "song-001",
            "Song One",
            200,
            artist_id,
            Some(album_id),
            Some(1),
            "/images/song1",
        )
        .await;
        insert_song(
            &pool,
            "song-002",
            "Song Two",
            210,
            artist_id,
            Some(album_id),
            Some(2),
            "/images/song2",
        )
        .await;
        insert_song(
            &pool,
            "song-003",
            "Song Three",
            220,
            artist_id,
            Some(album_id),
            Some(3),
            "/images/song3",
        )
        .await;

        let result = song_service::get_all_songs(&pool).await;
        assert!(result.is_ok());
        let songs = result.unwrap();
        assert_eq!(songs.len(), 3);

        let names: Vec<String> = songs.iter().map(|s| s.name.clone()).collect();
        assert!(names.contains(&"Song One".to_string()));
        assert!(names.contains(&"Song Two".to_string()));
        assert!(names.contains(&"Song Three".to_string()));
    }

    // ==================== GET BY ID ====================

    #[tokio::test]
    async fn test_get_song_by_id_not_found() {
        let pool = create_test_pool().await;

        let result = song_service::get_song_by_id(&pool, &uuid::Uuid::new_v4().to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_song_by_id_success() {
        let pool = create_test_pool().await;
        let artist_id = "artist-006";
        insert_artist(&pool, artist_id, "Test Artist").await;
        let expected_id = "song-find-me";
        insert_song(
            &pool,
            expected_id,
            "Find Me",
            300,
            artist_id,
            None,
            None,
            "/images/find_me",
        )
        .await;

        let result = song_service::get_song_by_id(&pool, expected_id).await;
        assert!(result.is_ok());
        let song = result.unwrap();
        assert_eq!(song.id, expected_id);
        assert_eq!(song.name, "Find Me");
        assert_eq!(song.duration, 300);
        assert_eq!(song.artist_id, artist_id);
    }

    // ==================== GET BY ARTIST ====================

    #[tokio::test]
    async fn test_get_songs_by_artist() {
        let pool = create_test_pool().await;
        let artist1_id = "artist-007";
        let artist2_id = "artist-008";
        insert_artist(&pool, artist1_id, "Artist One").await;
        insert_artist(&pool, artist2_id, "Artist Two").await;
        insert_song(
            &pool,
            "song-a1",
            "Artist 1 Song 1",
            200,
            artist1_id,
            None,
            Some(1),
            "/images/a1s1",
        )
        .await;
        insert_song(
            &pool,
            "song-a2",
            "Artist 1 Song 2",
            210,
            artist1_id,
            None,
            Some(2),
            "/images/a1s2",
        )
        .await;
        insert_song(
            &pool,
            "song-a3",
            "Artist 1 Song 3",
            220,
            artist1_id,
            None,
            Some(3),
            "/images/a1s3",
        )
        .await;
        insert_song(
            &pool,
            "song-b1",
            "Artist 2 Song 1",
            200,
            artist2_id,
            None,
            Some(1),
            "/images/a2s1",
        )
        .await;
        insert_song(
            &pool,
            "song-b2",
            "Artist 2 Song 2",
            210,
            artist2_id,
            None,
            Some(2),
            "/images/a2s2",
        )
        .await;

        let songs = song_service::get_songs_by_artist(&pool, artist1_id)
            .await
            .unwrap();
        assert_eq!(songs.len(), 3);

        let songs = song_service::get_songs_by_artist(&pool, artist2_id)
            .await
            .unwrap();
        assert_eq!(songs.len(), 2);
    }

    // ==================== GET BY ALBUM ====================

    #[tokio::test]
    async fn test_get_songs_by_album() {
        let pool = create_test_pool().await;
        let artist_id = "artist-009";
        let album_id = "album-009";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Test Album",
            0,
            artist_id,
            "/images/Test_Album",
            "ALBUM",
        )
        .await;
        insert_song(
            &pool,
            "song-1",
            "Album Song 1",
            200,
            artist_id,
            Some(album_id),
            Some(1),
            "/images/as1",
        )
        .await;
        insert_song(
            &pool,
            "song-2",
            "Album Song 2",
            210,
            artist_id,
            Some(album_id),
            Some(2),
            "/images/as2",
        )
        .await;
        insert_song(
            &pool,
            "song-3",
            "Album Song 3",
            220,
            artist_id,
            Some(album_id),
            Some(3),
            "/images/as3",
        )
        .await;
        insert_song(
            &pool,
            "song-4",
            "Album Song 4",
            230,
            artist_id,
            Some(album_id),
            Some(4),
            "/images/as4",
        )
        .await;

        let songs = song_service::get_songs_by_album(&pool, album_id)
            .await
            .unwrap();
        assert_eq!(songs.len(), 4);
    }

    // ==================== UPDATE ====================

    #[tokio::test]
    async fn test_update_song_success() {
        let pool = create_test_pool().await;
        let artist_id = "artist-010";
        let song_id = "song-update";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_song(
            &pool,
            song_id,
            "Original Name",
            180,
            artist_id,
            None,
            Some(1),
            "/images/original",
        )
        .await;

        let update = UpdateSong {
            name: Some("Updated Name".to_string()),
            duration: Some(240),
            release_date: None,
            track_number: Some(5),
        };

        let result = song_service::update_song(&pool, song_id, update).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_song_empty_name() {
        let pool = create_test_pool().await;
        let artist_id = "artist-011";
        let song_id = "song-empty-name";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_song(
            &pool,
            song_id,
            "Valid Name",
            180,
            artist_id,
            None,
            None,
            "/images/valid",
        )
        .await;

        let update = UpdateSong {
            name: Some(String::new()),
            duration: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::update_song(&pool, song_id, update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_song_not_found() {
        let pool = create_test_pool().await;

        let update = UpdateSong {
            name: Some("New Name".to_string()),
            duration: None,
            release_date: None,
            track_number: None,
        };

        let result =
            song_service::update_song(&pool, &uuid::Uuid::new_v4().to_string(), update).await;
        assert!(result.is_err());
    }

    // ==================== DELETE ====================

    #[tokio::test]
    async fn test_delete_song_success() {
        let pool = create_test_pool().await;
        let artist_id = "artist-012";
        let song_id = "song-delete";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_song(
            &pool,
            song_id,
            "To Delete",
            180,
            artist_id,
            None,
            None,
            "/images/delete",
        )
        .await;

        let result = song_service::delete_song(&pool, song_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_song_not_found() {
        let pool = create_test_pool().await;

        let result = song_service::delete_song(&pool, &uuid::Uuid::new_v4().to_string()).await;
        assert!(result.is_err());
    }

    // ==================== ALBUM TYPE ====================

    #[test]
    fn test_determine_album_type_single() {
        let album_type = song_service::determine_album_type(1, 180);
        assert_eq!(album_type, crate::models::song_model::AlbumType::Single);
    }

    #[test]
    fn test_determine_album_type_ep_by_count() {
        let album_type = song_service::determine_album_type(5, 600);
        assert_eq!(album_type, crate::models::song_model::AlbumType::Ep);
    }

    #[test]
    fn test_determine_album_type_ep_by_duration() {
        let album_type = song_service::determine_album_type(2, 1200);
        assert_eq!(album_type, crate::models::song_model::AlbumType::Ep);
    }

    #[test]
    fn test_determine_album_type_album_by_count() {
        let album_type = song_service::determine_album_type(7, 1000);
        assert_eq!(album_type, crate::models::song_model::AlbumType::Album);
    }

    #[test]
    fn test_determine_album_type_album_by_duration() {
        let album_type = song_service::determine_album_type(3, 2000);
        assert_eq!(album_type, crate::models::song_model::AlbumType::Album);
    }

    // ==================== SIDE EFFECTS ====================

    #[tokio::test]
    async fn test_standalone_collection_created_for_song_without_album() {
        let pool = create_test_pool().await;
        let artist_id = "artist-013";
        insert_artist(&pool, artist_id, "Standalone Artist").await;

        let song = CreateSong {
            name: "Standalone".to_string(),
            duration: 180,
            artist_id: artist_id.to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        song_service::create_song(&pool, song).await.unwrap();

        let client = pool.get().await.expect("Failed to get client");
        let album_exists: bool = client
            .query_one(
                "SELECT 1 FROM albums WHERE artist_id = $1 AND album_type = 'STANDALONE_COLLECTION'",
                &[&artist_id],
            )
            .await
            .is_ok();
        assert!(album_exists);
    }

    #[tokio::test]
    async fn test_album_stats_updated_on_song_create() {
        let pool = create_test_pool().await;
        let artist_id = "artist-014";
        let album_id = "album-014";
        insert_artist(&pool, artist_id, "Test Artist").await;
        insert_album(
            &pool,
            album_id,
            "Test Album",
            0,
            artist_id,
            "/images/Test_Album",
            "ALBUM",
        )
        .await;

        for i in 1..=3 {
            let song = CreateSong {
                name: format!("Song {}", i),
                duration: 300,
                artist_id: artist_id.to_string(),
                album_id: Some(album_id.to_string()),
                release_date: None,
                track_number: Some(i),
            };
            song_service::create_song(&pool, song).await.unwrap();
        }

        let client = pool.get().await.expect("Failed to get client");
        let row = client
            .query_one(
                "SELECT COUNT(*), COALESCE(SUM(duration), 0) FROM songs WHERE album_id = $1",
                &[&album_id],
            )
            .await
            .expect("Query failed");
        let count: i64 = row.get(0);
        let total_duration: i64 = row.get(1);
        assert_eq!(count, 3);
        assert_eq!(total_duration, 900);

        let album_type: String = client
            .query_one("SELECT album_type FROM albums WHERE id = $1", &[&album_id])
            .await
            .expect("Query failed")
            .get(0);
        assert_eq!(album_type, "EP");
    }
}
