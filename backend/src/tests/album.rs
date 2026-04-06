#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rusqlite::{params, Connection};
    use uuid::Uuid;

    use crate::models::album_model::{CreateAlbum, UpdateAlbum};
    use crate::models::song_model::AlbumType;
    use crate::services::album_service;

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
    fn test_create_album_success() {
        let db = create_test_db();
        let artist_id = "artist-001";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();

        let album = CreateAlbum {
            name: "Test Album".to_string(),
            release_date: NaiveDate::from_ymd_opt(2024, 6, 15),
            artist_id: artist_id.to_string(),
            album_type: Some(AlbumType::Album),
        };

        let result = album_service::create_album(&db, album);

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM albums WHERE name = ?1 AND artist_id = ?2",
                params!["Test Album", artist_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        let (id, name, total_duration, artist_id_result, album_type): (String, String, i32, String, String) = db
            .query_row(
                "SELECT id, name, total_duration, artist_id, album_type FROM albums WHERE name = ?1",
                params!["Test Album"],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
            )
            .unwrap();
        assert_eq!(name, "Test Album");
        assert_eq!(total_duration, 0);
        assert_eq!(artist_id_result, artist_id);
        assert_eq!(album_type, "ALBUM");
        assert!(!id.is_empty());
    }

    #[test]
    fn test_create_album_empty_name() {
        let db = create_test_db();
        let artist_id = "artist-002";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();

        let album = CreateAlbum {
            name: "".to_string(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&db, album);

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM albums", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_create_album_whitespace_name() {
        let db = create_test_db();
        let artist_id = "artist-003";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();

        let album = CreateAlbum {
            name: "   ".to_string(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&db, album);

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM albums", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_create_album_invalid_artist() {
        let db = create_test_db();

        let album = CreateAlbum {
            name: "Test Album".to_string(),
            release_date: None,
            artist_id: "nonexistent-artist".to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&db, album);

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM albums", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_create_album_duplicate_for_artist() {
        let db = create_test_db();
        let artist_id = "artist-004";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["existing-album", "Duplicate Album", 0, artist_id, "/images/existing", "ALBUM"],
        )
        .unwrap();

        let album = CreateAlbum {
            name: "Duplicate Album".to_string(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&db, album);

        assert!(result.is_err());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM albums WHERE name = ?1 AND artist_id = ?2",
                params!["Duplicate Album", artist_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_create_album_same_name_different_artist() {
        let db = create_test_db();
        let artist1_id = "artist-005a";
        let artist2_id = "artist-005b";
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
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-a", "Shared Name", 0, artist1_id, "/images/a", "ALBUM"],
        )
        .unwrap();

        let album = CreateAlbum {
            name: "Shared Name".to_string(),
            release_date: None,
            artist_id: artist2_id.to_string(),
            album_type: None,
        };

        let result = album_service::create_album(&db, album);

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM albums WHERE name = ?1",
                params!["Shared Name"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_create_album_default_type() {
        let db = create_test_db();
        let artist_id = "artist-006";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();

        let album = CreateAlbum {
            name: "Default Type Album".to_string(),
            release_date: None,
            artist_id: artist_id.to_string(),
            album_type: None,
        };

        album_service::create_album(&db, album).unwrap();

        let album_type: String = db
            .query_row(
                "SELECT album_type FROM albums WHERE name = ?1",
                params!["Default Type Album"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(album_type, "ALBUM");
    }

    // ==================== GET ALL ====================

    #[test]
    fn test_get_all_albums_empty() {
        let db = create_test_db();

        let result = album_service::get_all_albums(&db);

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_get_all_albums_with_data() {
        let db = create_test_db();
        let artist_id = "artist-007";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-1", "Album One", 1200, artist_id, "/images/one", "ALBUM"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-2", "Album Two", 600, artist_id, "/images/two", "EP"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-3", "Album Three", 180, artist_id, "/images/three", "SINGLE"],
        )
        .unwrap();

        let result = album_service::get_all_albums(&db);

        assert!(result.is_ok());
        let albums = result.unwrap();
        assert_eq!(albums.len(), 3);

        let names: Vec<String> = albums.iter().map(|a| a.name.clone()).collect();
        assert!(names.contains(&"Album One".to_string()));
        assert!(names.contains(&"Album Two".to_string()));
        assert!(names.contains(&"Album Three".to_string()));
    }

    // ==================== GET BY ID ====================

    #[test]
    fn test_get_album_by_id_not_found() {
        let db = create_test_db();

        let result = album_service::get_album_by_id(&db, &Uuid::new_v4().to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_get_album_by_id_success() {
        let db = create_test_db();
        let artist_id = "artist-008";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        let expected_id = "album-find-me";
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![expected_id, "Find Me Album", 900, artist_id, "/images/find_me", "EP"],
        )
        .unwrap();

        let result = album_service::get_album_by_id(&db, expected_id);

        assert!(result.is_ok());
        let album = result.unwrap();
        assert_eq!(album.id, expected_id);
        assert_eq!(album.name, "Find Me Album");
        assert_eq!(album.total_duration, 900);
        assert_eq!(album.artist_id, artist_id);
        assert_eq!(album.album_type, AlbumType::Ep);
    }

    // ==================== GET BY ARTIST ====================

    #[test]
    fn test_get_albums_by_artist() {
        let db = create_test_db();
        let artist1_id = "artist-009";
        let artist2_id = "artist-010";
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
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-a1", "Artist 1 Album 1", 1200, artist1_id, "/images/a1b1", "ALBUM"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-a2", "Artist 1 Album 2", 600, artist1_id, "/images/a1b2", "EP"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-a3", "Artist 1 Album 3", 180, artist1_id, "/images/a1b3", "SINGLE"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-b1", "Artist 2 Album 1", 1500, artist2_id, "/images/a2b1", "ALBUM"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params!["album-b2", "Artist 2 Album 2", 300, artist2_id, "/images/a2b2", "SINGLE"],
        )
        .unwrap();

        let albums = album_service::get_albums_by_artist(&db, artist1_id).unwrap();
        assert_eq!(albums.len(), 3);

        let albums = album_service::get_albums_by_artist(&db, artist2_id).unwrap();
        assert_eq!(albums.len(), 2);
    }

    // ==================== UPDATE ====================

    #[test]
    fn test_update_album_name() {
        let db = create_test_db();
        let artist_id = "artist-011";
        let album_id = "album-update-name";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Old Name", 0, artist_id, "/images/old", "ALBUM"],
        )
        .unwrap();

        let update = UpdateAlbum {
            name: Some("Updated Name".to_string()),
            release_date: None,
            artist_id: None,
        };

        let result = album_service::update_album(&db, album_id, update);

        assert!(result.is_ok());

        let name: String = db
            .query_row(
                "SELECT name FROM albums WHERE id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(name, "Updated Name");
    }

    #[test]
    fn test_update_album_empty_name() {
        let db = create_test_db();
        let artist_id = "artist-012";
        let album_id = "album-empty-name";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Valid Name", 0, artist_id, "/images/valid", "ALBUM"],
        )
        .unwrap();

        let update = UpdateAlbum {
            name: Some("".to_string()),
            release_date: None,
            artist_id: None,
        };

        let result = album_service::update_album(&db, album_id, update);

        assert!(result.is_err());

        let name: String = db
            .query_row(
                "SELECT name FROM albums WHERE id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(name, "Valid Name");
    }

    #[test]
    fn test_update_album_artist() {
        let db = create_test_db();
        let artist1_id = "artist-013a";
        let artist2_id = "artist-013b";
        let album_id = "album-update-artist";
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
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Transfer Album", 0, artist1_id, "/images/transfer", "ALBUM"],
        )
        .unwrap();

        let update = UpdateAlbum {
            name: None,
            release_date: None,
            artist_id: Some(artist2_id.to_string()),
        };

        let result = album_service::update_album(&db, album_id, update);

        assert!(result.is_ok());

        let artist_id_result: String = db
            .query_row(
                "SELECT artist_id FROM albums WHERE id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(artist_id_result, artist2_id);
    }

    #[test]
    fn test_update_album_invalid_artist() {
        let db = create_test_db();
        let artist_id = "artist-014";
        let album_id = "album-invalid-artist";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Test Album", 0, artist_id, "/images/test", "ALBUM"],
        )
        .unwrap();

        let update = UpdateAlbum {
            name: None,
            release_date: None,
            artist_id: Some("nonexistent-artist".to_string()),
        };

        let result = album_service::update_album(&db, album_id, update);

        assert!(result.is_err());

        let artist_id_result: String = db
            .query_row(
                "SELECT artist_id FROM albums WHERE id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(artist_id_result, artist_id);
    }

    #[test]
    fn test_update_album_not_found() {
        let db = create_test_db();

        let update = UpdateAlbum {
            name: Some("New Name".to_string()),
            release_date: None,
            artist_id: None,
        };

        let result = album_service::update_album(&db, &Uuid::new_v4().to_string(), update);

        assert!(result.is_err());
    }

    #[test]
    fn test_update_album_no_changes() {
        let db = create_test_db();
        let artist_id = "artist-015";
        let album_id = "album-no-changes";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Unchanged", 500, artist_id, "/images/unchanged", "ALBUM"],
        )
        .unwrap();

        let update = UpdateAlbum {
            name: None,
            release_date: None,
            artist_id: None,
        };

        let result = album_service::update_album(&db, album_id, update);

        assert!(result.is_ok());

        let (name, total_duration): (String, i32) = db
            .query_row(
                "SELECT name, total_duration FROM albums WHERE id = ?1",
                params![album_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(name, "Unchanged");
        assert_eq!(total_duration, 500);
    }

    // ==================== DELETE ====================

    #[test]
    fn test_delete_album_success() {
        let db = create_test_db();
        let artist_id = "artist-016";
        let album_id = "album-delete";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "To Delete", 0, artist_id, "/images/delete", "ALBUM"],
        )
        .unwrap();

        let result = album_service::delete_album(&db, album_id);

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM albums WHERE id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_album_not_found() {
        let db = create_test_db();

        let result = album_service::delete_album(&db, &Uuid::new_v4().to_string());

        assert!(result.is_err());
    }

    // ==================== UPDATE DURATION ====================

    #[test]
    fn test_update_album_duration() {
        let db = create_test_db();
        let artist_id = "artist-017";
        let album_id = "album-duration";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Test Artist", "/images/Test_Artist"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO albums (id, name, total_duration, artist_id, image_path, album_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![album_id, "Duration Album", 0, artist_id, "/images/duration", "ALBUM"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-d1", "Song 1", 200, artist_id, album_id, None::<i64>, 1, "/images/d1"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-d2", "Song 2", 300, artist_id, album_id, None::<i64>, 2, "/images/d2"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO songs (id, name, duration, artist_id, album_id, release_date, track_number, image_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params!["song-d3", "Song 3", 250, artist_id, album_id, None::<i64>, 3, "/images/d3"],
        )
        .unwrap();

        let result = album_service::update_album_duration(&db, album_id);

        assert!(result.is_ok());

        let total_duration: i32 = db
            .query_row(
                "SELECT total_duration FROM albums WHERE id = ?1",
                params![album_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(total_duration, 750);
    }
}
