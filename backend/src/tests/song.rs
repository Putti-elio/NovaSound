#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rusqlite::{params, Connection};
    use uuid::Uuid;

    use crate::models::song_model::{CreateSong, UpdateSong};
    use crate::services::song_service;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE artists (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                image_path TEXT
            );

            CREATE TABLE albums (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                total_duration INTEGER DEFAULT 0,
                release_date INTEGER,
                artist_id TEXT NOT NULL,
                image_path TEXT,
                album_type TEXT DEFAULT 'ALBUM',
                FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
            );

            CREATE TABLE songs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                duration INTEGER,
                artist_id TEXT NOT NULL,
                album_id TEXT,
                release_date INTEGER,
                track_number INTEGER,
                image_path TEXT,
                FOREIGN KEY (artist_id) REFERENCES artists(id),
                FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE SET NULL
            );

            CREATE INDEX IF NOT EXISTS idx_songs_album_id ON songs(album_id);
            CREATE INDEX IF NOT EXISTS idx_songs_artist_id ON songs(artist_id);
            ",
        )
        .unwrap();
        conn
    }

    // ==================== CREATE ====================

    #[test]
    fn test_create_song_success_with_album() {
        let db = create_test_db();
        let artist_id = "artist-001";
        let album_id = "album-001";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Test Album", 0, artist_id, "/images/Test_Album", "ALBUM"],
        )
        .unwrap();

        let song = CreateSong {
            name: "Test Song".to_string(),
            duration: 240,
            artist_id: artist_id.to_string(),
            album_id: Some(album_id.to_string()),
            release_date: NaiveDate::from_ymd_opt(2024, 1, 1),
            track_number: Some(1),
        };

        let result = song_service::create_song(&db, song);

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM songs WHERE name = ?1",
                params!["Test Song"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        let (name, duration, album_id_result): (String, i32, String) = db
            .query_row(
                "SELECT name, duration, album_id FROM songs WHERE name = ?1",
                params!["Test Song"],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(name, "Test Song");
        assert_eq!(duration, 240);
        assert_eq!(album_id_result, album_id);
    }

    #[test]
    fn test_create_song_success_without_album() {
        let db = create_test_db();
        let artist_id = "artist-002";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Solo Artist", "/images/Solo_Artist"],
        )
        .unwrap();

        let song = CreateSong {
            name: "Standalone Song".to_string(),
            duration: 180,
            artist_id: artist_id.to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&db, song);

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM songs WHERE name = ?1",
                params!["Standalone Song"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        let album_exists: bool = db
            .query_row(
                "SELECT 1 FROM albums WHERE artist_id = ?1 AND album_type = 'STANDALONE_COLLECTION'",
                params![artist_id],
                |_| Ok(true),
            )
            .unwrap_or(false);
        assert!(album_exists);
    }

    #[test]
    fn test_create_song_empty_name() {
        let db = create_test_db();
        let artist_id = "artist-003";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();

        let song = CreateSong {
            name: "".to_string(),
            duration: 240,
            artist_id: artist_id.to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&db, song);

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM songs", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_create_song_invalid_artist() {
        let db = create_test_db();

        let song = CreateSong {
            name: "Test Song".to_string(),
            duration: 240,
            artist_id: "nonexistent-artist".to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&db, song);

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM songs", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_create_song_invalid_album() {
        let db = create_test_db();
        let artist_id = "artist-004";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();

        let song = CreateSong {
            name: "Test Song".to_string(),
            duration: 240,
            artist_id: artist_id.to_string(),
            album_id: Some("nonexistent-album".to_string()),
            release_date: None,
            track_number: None,
        };

        let result = song_service::create_song(&db, song);

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM songs", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    // ==================== GET ALL ====================

    #[test]
    fn test_get_all_songs_empty() {
        let db = create_test_db();

        let result = song_service::get_all_songs(&db);

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_get_all_songs_with_data() {
        let db = create_test_db();
        let artist_id = "artist-005";
        let album_id = "album-005";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Test Album", 0, artist_id, "/images/Test_Album", "ALBUM"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-001", "Song One", 200, artist_id, album_id, None::<i64>, 1, "/images/song1"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-002", "Song Two", 210, artist_id, album_id, None::<i64>, 2, "/images/song2"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-003", "Song Three", 220, artist_id, album_id, None::<i64>, 3, "/images/song3"],
        )
        .unwrap();

        let result = song_service::get_all_songs(&db);

        assert!(result.is_ok());
        let songs = result.unwrap();
        assert_eq!(songs.len(), 3);

        let names: Vec<String> = songs.iter().map(|s| s.name.clone()).collect();
        assert!(names.contains(&"Song One".to_string()));
        assert!(names.contains(&"Song Two".to_string()));
        assert!(names.contains(&"Song Three".to_string()));
    }

    // ==================== GET BY ID ====================

    #[test]
    fn test_get_song_by_id_not_found() {
        let db = create_test_db();

        let result = song_service::get_song_by_id(&db, &Uuid::new_v4().to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_get_song_by_id_success() {
        let db = create_test_db();
        let artist_id = "artist-006";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        let expected_id = "song-find-me";
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![expected_id, "Find Me", 300, artist_id, None::<String>, None::<i64>, None::<i32>, "/images/find_me"],
        )
        .unwrap();

        let result = song_service::get_song_by_id(&db, expected_id);

        assert!(result.is_ok());
        let song = result.unwrap();
        assert_eq!(song.id, expected_id);
        assert_eq!(song.name, "Find Me");
        assert_eq!(song.duration, 300);
        assert_eq!(song.artist_id, artist_id);
    }

    // ==================== GET BY ARTIST ====================

    #[test]
    fn test_get_songs_by_artist() {
        let db = create_test_db();
        let artist1_id = "artist-007";
        let artist2_id = "artist-008";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist1_id, "Artist One", "/images/Artist_One"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist2_id, "Artist Two", "/images/Artist_Two"],
        )
        .unwrap();

        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-a1", "Artist 1 Song 1", 200, artist1_id, None::<String>, None::<i64>, 1, "/images/a1s1"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-a2", "Artist 1 Song 2", 210, artist1_id, None::<String>, None::<i64>, 2, "/images/a1s2"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-a3", "Artist 1 Song 3", 220, artist1_id, None::<String>, None::<i64>, 3, "/images/a1s3"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-b1", "Artist 2 Song 1", 200, artist2_id, None::<String>, None::<i64>, 1, "/images/a2s1"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-b2", "Artist 2 Song 2", 210, artist2_id, None::<String>, None::<i64>, 2, "/images/a2s2"],
        )
        .unwrap();

        let songs = song_service::get_songs_by_artist(&db, artist1_id).unwrap();
        assert_eq!(songs.len(), 3);

        let songs = song_service::get_songs_by_artist(&db, artist2_id).unwrap();
        assert_eq!(songs.len(), 2);
    }

    // ==================== GET BY ALBUM ====================

    #[test]
    fn test_get_songs_by_album() {
        let db = create_test_db();
        let artist_id = "artist-009";
        let album_id = "album-009";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Test Album", 0, artist_id, "/images/Test_Album", "ALBUM"],
        )
        .unwrap();

        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-1", "Album Song 1", 200, artist_id, album_id, None::<i64>, 1, "/images/as1"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-2", "Album Song 2", 210, artist_id, album_id, None::<i64>, 2, "/images/as2"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-3", "Album Song 3", 220, artist_id, album_id, None::<i64>, 3, "/images/as3"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-4", "Album Song 4", 230, artist_id, album_id, None::<i64>, 4, "/images/as4"],
        )
        .unwrap();

        let songs = song_service::get_songs_by_album(&db, album_id).unwrap();
        assert_eq!(songs.len(), 4);

        let album_song_count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM songs WHERE album_id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(album_song_count, 4);
    }

    // ==================== UPDATE ====================

    #[test]
    fn test_update_song_success() {
        let db = create_test_db();
        let artist_id = "artist-010";
        let song_id = "song-update";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![song_id, "Original Name", 180, artist_id, None::<String>, None::<i64>, 1, "/images/original"],
        )
        .unwrap();

        let update = UpdateSong {
            name: Some("Updated Name".to_string()),
            duration: Some(240),
            release_date: None,
            track_number: Some(5),
        };

        let result = song_service::update_song(&db, song_id, update);

        assert!(result.is_ok());

        let (name, duration, track_number): (String, i32, Option<i32>) = db
            .query_row(
                "SELECT name, duration, track_number FROM songs WHERE id = ?1",
                params![song_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(name, "Updated Name");
        assert_eq!(duration, 240);
        assert_eq!(track_number, Some(5));
    }

    #[test]
    fn test_update_song_empty_name() {
        let db = create_test_db();
        let artist_id = "artist-011";
        let song_id = "song-empty-name";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![song_id, "Valid Name", 180, artist_id, None::<String>, None::<i64>, None::<i32>, "/images/valid"],
        )
        .unwrap();

        let update = UpdateSong {
            name: Some("".to_string()),
            duration: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::update_song(&db, song_id, update);

        assert!(result.is_err());

        let name: String = db
            .query_row(
                "SELECT name FROM songs WHERE id = ?1",
                params![song_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(name, "Valid Name");
    }

    #[test]
    fn test_update_song_not_found() {
        let db = create_test_db();

        let update = UpdateSong {
            name: Some("New Name".to_string()),
            duration: None,
            release_date: None,
            track_number: None,
        };

        let result = song_service::update_song(&db, &Uuid::new_v4().to_string(), update);

        assert!(result.is_err());
    }

    // ==================== DELETE ====================

    #[test]
    fn test_delete_song_success() {
        let db = create_test_db();
        let artist_id = "artist-012";
        let song_id = "song-delete";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![song_id, "To Delete", 180, artist_id, None::<String>, None::<i64>, None::<i32>, "/images/delete"],
        )
        .unwrap();

        let result = song_service::delete_song(&db, song_id);

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM songs WHERE id = ?1",
                params![song_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_song_not_found() {
        let db = create_test_db();

        let result = song_service::delete_song(&db, &Uuid::new_v4().to_string());

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

    #[test]
    fn test_standalone_collection_created_for_song_without_album() {
        let db = create_test_db();
        let artist_id = "artist-013";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Standalone Artist", "/images/Standalone_Artist"],
        )
        .unwrap();

        let song = CreateSong {
            name: "Standalone".to_string(),
            duration: 180,
            artist_id: artist_id.to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        };

        song_service::create_song(&db, song).unwrap();

        let album_exists: bool = db
            .query_row(
                "SELECT 1 FROM albums WHERE artist_id = ?1 AND album_type = 'STANDALONE_COLLECTION'",
                params![artist_id],
                |_| Ok(true),
            )
            .unwrap_or(false);
        assert!(album_exists);
    }

    #[test]
    fn test_album_stats_updated_on_song_create() {
        let db = create_test_db();
        let artist_id = "artist-014";
        let album_id = "album-014";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Test Album", 0, artist_id, "/images/Test_Album", "ALBUM"],
        )
        .unwrap();

        for i in 1..=3 {
            let song = CreateSong {
                name: format!("Song {}", i),
                duration: 300,
                artist_id: artist_id.to_string(),
                album_id: Some(album_id.to_string()),
                release_date: None,
                track_number: Some(i),
            };
            song_service::create_song(&db, song).unwrap();
        }

        let (count, total_duration): (i32, i32) = db
            .query_row(
                "SELECT COUNT(*), COALESCE(SUM(duration), 0) FROM songs WHERE album_id = ?1",
                params![album_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(count, 3);
        assert_eq!(total_duration, 900);

        let album_type: String = db
            .query_row(
                "SELECT album_type FROM albums WHERE id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(album_type, "EP");
    }
}
