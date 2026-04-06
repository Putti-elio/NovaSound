#[cfg(test)]
mod tests {
    use rusqlite::{params, Connection};
    use uuid::Uuid;

    use crate::services::artist_service;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE artists (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                image_path TEXT
            );
            ",
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_create_artist_success() {
        let db = create_test_db();

        let result = artist_service::create_artist(&db, "Test Artist");

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM artists WHERE name = ?1",
                params!["Test Artist"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        let (id, name, image_path): (String, String, String) = db
            .query_row(
                "SELECT id, name, image_path FROM artists WHERE name = ?1",
                params!["Test Artist"],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(name, "Test Artist");
        assert_eq!(image_path, "/images/Test Artist");
        assert!(!id.is_empty());
    }

    #[test]
    fn test_create_artist_empty_name() {
        let db = create_test_db();

        let result = artist_service::create_artist(&db, "");

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM artists", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_create_artist_whitespace_name() {
        let db = create_test_db();

        let result = artist_service::create_artist(&db, "   ");

        assert!(result.is_err());

        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM artists", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_create_artist_duplicate_name() {
        let db = create_test_db();
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![
                "existing-id",
                "Duplicate Artist",
                "/images/Duplicate Artist"
            ],
        )
        .unwrap();

        let result = artist_service::create_artist(&db, "Duplicate Artist");

        assert!(result.is_err());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM artists WHERE name = ?1",
                params!["Duplicate Artist"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_all_artists_empty() {
        let db = create_test_db();

        let result = artist_service::get_all_artists(&db);

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_get_all_artists_with_data() {
        let db = create_test_db();
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params!["id-1", "Artist One", "/images/Artist One"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params!["id-2", "Artist Two", "/images/Artist Two"],
        )
        .unwrap();
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params!["id-3", "Artist Three", "/images/Artist Three"],
        )
        .unwrap();

        let result = artist_service::get_all_artists(&db);

        assert!(result.is_ok());
        let artists = result.unwrap();
        assert_eq!(artists.len(), 3);

        let names: Vec<String> = artists.iter().map(|a| a.name.clone()).collect();
        assert!(names.contains(&"Artist One".to_string()));
        assert!(names.contains(&"Artist Two".to_string()));
        assert!(names.contains(&"Artist Three".to_string()));
    }

    #[test]
    fn test_get_artist_by_id_not_found() {
        let db = create_test_db();

        let result = artist_service::get_artist(&db, &Uuid::new_v4().to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_get_artist_by_id_success() {
        let db = create_test_db();
        let expected_id = "test-uuid-123";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![expected_id, "Fetched Artist", "/images/Fetched Artist"],
        )
        .unwrap();

        let result = artist_service::get_artist(&db, &expected_id.to_string());

        assert!(result.is_ok());
        let artist = result.unwrap();
        assert_eq!(artist.id, expected_id);
        assert_eq!(artist.name, "Fetched Artist");
        assert_eq!(artist.image_path, "/images/Fetched Artist");
    }

    #[test]
    fn test_update_artist_success() {
        let db = create_test_db();
        let artist_id = "update-uuid-456";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Old Name", "/images/Old Name"],
        )
        .unwrap();

        let result = artist_service::update_artist(&db, artist_id, "New Name");

        assert!(result.is_ok());

        let (name, image_path): (String, String) = db
            .query_row(
                "SELECT name, image_path FROM artists WHERE id = ?1",
                params![artist_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(name, "New Name");
        assert_eq!(image_path, "/images/New Name");
    }

    #[test]
    fn test_update_artist_empty_name() {
        let db = create_test_db();
        let artist_id = "update-uuid-789";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "Valid Name", "/images/Valid Name"],
        )
        .unwrap();

        let result = artist_service::update_artist(&db, artist_id, "");

        assert!(result.is_err());

        let name: String = db
            .query_row(
                "SELECT name FROM artists WHERE id = ?1",
                params![artist_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(name, "Valid Name");
    }

    #[test]
    fn test_update_artist_not_found() {
        let db = create_test_db();

        let result = artist_service::update_artist(&db, &Uuid::new_v4().to_string(), "New Name");

        assert!(result.is_err());
    }

    #[test]
    fn test_delete_artist_success() {
        let db = create_test_db();
        let artist_id = "delete-uuid-012";
        db.execute(
            "INSERT INTO artists (id, name, image_path) VALUES (?1, ?2, ?3)",
            params![artist_id, "To Delete", "/images/To Delete"],
        )
        .unwrap();

        let result = artist_service::delete_artist(&db, artist_id);

        assert!(result.is_ok());

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM artists WHERE id = ?1",
                params![artist_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_artist_not_found() {
        let db = create_test_db();

        let result = artist_service::delete_artist(&db, &Uuid::new_v4().to_string());

        assert!(result.is_err());
    }
}
