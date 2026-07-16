#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::too_many_arguments)]
    use crate::get_test_pool;
    use crate::services::artist_service;

    #[tokio::test]
    async fn test_create_artist_success() {
        let pool = get_test_pool!();

        let result = artist_service::create_artist(&pool, "Test Artist").await;
        assert!(result.is_ok());

        let client = pool.get().await.expect("Failed to get client");
        let count: i64 = client
            .query_one(
                "SELECT COUNT(*) FROM artists WHERE name = $1",
                &[&"Test Artist"],
            )
            .await
            .expect("Query failed")
            .get(0);
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_create_artist_empty_name() {
        let pool = get_test_pool!();

        let result = artist_service::create_artist(&pool, "").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_artist_whitespace_name() {
        let pool = get_test_pool!();

        let result = artist_service::create_artist(&pool, " ").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_artist_duplicate_name() {
        let pool = get_test_pool!();
        let client = pool.get().await.expect("Failed to get client");
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[
                    &"existing-id",
                    &"Duplicate Artist",
                    &"/images/Duplicate Artist",
                ],
            )
            .await
            .expect("Insert failed");

        let result = artist_service::create_artist(&pool, "Duplicate Artist").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_all_artists_empty() {
        let pool = get_test_pool!();

        let result = artist_service::get_all_artists(&pool).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_all_artists_with_data() {
        let pool = get_test_pool!();
        let client = pool.get().await.expect("Failed to get client");
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&"id-1", &"Artist One", &"/images/Artist One"],
            )
            .await
            .expect("Insert failed");
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&"id-2", &"Artist Two", &"/images/Artist Two"],
            )
            .await
            .expect("Insert failed");
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&"id-3", &"Artist Three", &"/images/Artist Three"],
            )
            .await
            .expect("Insert failed");

        let result = artist_service::get_all_artists(&pool).await;
        assert!(result.is_ok());
        let artists = result.unwrap();
        assert_eq!(artists.len(), 3);

        let names: Vec<String> = artists.iter().map(|a| a.name.clone()).collect();
        assert!(names.contains(&"Artist One".to_string()));
        assert!(names.contains(&"Artist Two".to_string()));
        assert!(names.contains(&"Artist Three".to_string()));
    }

    #[tokio::test]
    async fn test_get_artist_by_id_not_found() {
        let pool = get_test_pool!();

        let result = artist_service::get_artist(&pool, &uuid::Uuid::new_v4().to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_artist_by_id_success() {
        let pool = get_test_pool!();
        let client = pool.get().await.expect("Failed to get client");
        let expected_id = "test-uuid-123";
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&expected_id, &"Fetched Artist", &"/images/Fetched Artist"],
            )
            .await
            .expect("Insert failed");

        let result = artist_service::get_artist(&pool, expected_id).await;
        assert!(result.is_ok());
        let artist = result.unwrap();
        assert_eq!(artist.id, expected_id);
        assert_eq!(artist.name, "Fetched Artist");
        assert_eq!(artist.image_path, "/images/Fetched Artist");
    }

    #[tokio::test]
    async fn test_update_artist_success() {
        let pool = get_test_pool!();
        let client = pool.get().await.expect("Failed to get client");
        let artist_id = "update-uuid-456";
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&artist_id, &"Old Name", &"/images/Old Name"],
            )
            .await
            .expect("Insert failed");

        let result = artist_service::update_artist(&pool, artist_id, "New Name").await;
        assert!(result.is_ok());

        let row = client
            .query_one(
                "SELECT name, image_path FROM artists WHERE id = $1",
                &[&artist_id],
            )
            .await
            .expect("Query failed");
        let name: String = row.get(0);
        let image_path: String = row.get(1);
        assert_eq!(name, "New Name");
        assert_eq!(image_path, "/images/New Name");
    }

    #[tokio::test]
    async fn test_update_artist_empty_name() {
        let pool = get_test_pool!();
        let client = pool.get().await.expect("Failed to get client");
        let artist_id = "update-uuid-789";
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&artist_id, &"Valid Name", &"/images/Valid Name"],
            )
            .await
            .expect("Insert failed");

        let result = artist_service::update_artist(&pool, artist_id, "").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_artist_not_found() {
        let pool = get_test_pool!();

        let result =
            artist_service::update_artist(&pool, &uuid::Uuid::new_v4().to_string(), "New Name")
                .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_artist_success() {
        let pool = get_test_pool!();
        let client = pool.get().await.expect("Failed to get client");
        let artist_id = "delete-uuid-012";
        client
            .execute(
                "INSERT INTO artists (id, name, image_path) VALUES ($1, $2, $3)",
                &[&artist_id, &"To Delete", &"/images/To Delete"],
            )
            .await
            .expect("Insert failed");

        let result = artist_service::delete_artist(&pool, artist_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_artist_not_found() {
        let pool = get_test_pool!();

        let result = artist_service::delete_artist(&pool, &uuid::Uuid::new_v4().to_string()).await;
        assert!(result.is_err());
    }
}
