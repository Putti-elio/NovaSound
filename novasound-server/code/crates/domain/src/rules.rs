use crate::models::song_model::AlbumType;

const STANDALONE_COLLECTION_SUFFIX: &str = "Standalone Collection";

#[derive(Debug, PartialEq, Eq)]
pub struct StandaloneCollection {
    pub name: String,
    pub album_type: AlbumType,
}

#[must_use]
pub fn has_non_empty_name(name: &str) -> bool {
    !name.trim().is_empty()
}

#[must_use]
pub fn determine_album_type(song_count: i32, total_duration: u32) -> AlbumType {
    if song_count >= 7 || total_duration >= 1800 {
        AlbumType::Album
    } else if (4..=6).contains(&song_count) || (900..1800).contains(&total_duration) {
        AlbumType::Ep
    } else {
        AlbumType::Single
    }
}

#[must_use]
pub fn standalone_collection_for_artist(artist_name: &str) -> StandaloneCollection {
    StandaloneCollection {
        name: format!("{artist_name} {STANDALONE_COLLECTION_SUFFIX}"),
        album_type: AlbumType::StandaloneCollection,
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{determine_album_type, has_non_empty_name, standalone_collection_for_artist};
    use crate::models::{
        album_model::{Album, UpdateAlbum},
        song_model::{AlbumType, Song, UpdateSong},
    };

    #[test]
    fn name_must_contain_non_whitespace_characters() {
        assert!(!has_non_empty_name(""));
        assert!(!has_non_empty_name(" \t\n"));
        assert!(has_non_empty_name("NovaSound"));
    }

    #[test]
    fn classifies_albums_at_count_and_duration_boundaries() {
        let cases = [
            (3, 899, AlbumType::Single),
            (4, 899, AlbumType::Ep),
            (6, 899, AlbumType::Ep),
            (7, 899, AlbumType::Album),
            (3, 900, AlbumType::Ep),
            (3, 1799, AlbumType::Ep),
            (3, 1800, AlbumType::Album),
        ];

        for (song_count, total_duration, expected_type) in cases {
            assert_eq!(
                determine_album_type(song_count, total_duration),
                expected_type
            );
        }
    }

    #[test]
    fn derives_standalone_collection_name_and_type() {
        assert_eq!(
            standalone_collection_for_artist("Solo Artist"),
            super::StandaloneCollection {
                name: "Solo Artist Standalone Collection".to_string(),
                album_type: AlbumType::StandaloneCollection,
            }
        );
    }

    #[test]
    fn song_merge_preserves_omitted_fields() {
        let song = Song {
            id: "song-id".to_string(),
            name: "Original".to_string(),
            duration: 180,
            artist_id: "artist-id".to_string(),
            album_id: Some("album-id".to_string()),
            release_date: NaiveDate::from_ymd_opt(2024, 1, 1),
            track_number: Some(1),
            image_path: Some("/images/song".to_string()),
        };

        let updated = song.merge_update(UpdateSong {
            name: Some("Updated".to_string()),
            duration: Some(240),
            release_date: None,
            track_number: Some(2),
        });

        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.duration, 240);
        assert_eq!(updated.release_date, NaiveDate::from_ymd_opt(2024, 1, 1));
        assert_eq!(updated.track_number, Some(2));
        assert_eq!(updated.artist_id, "artist-id");
        assert_eq!(updated.album_id.as_deref(), Some("album-id"));
        assert!(
            !UpdateSong {
                name: None,
                duration: None,
                release_date: None,
                track_number: None,
            }
            .has_changes()
        );
    }

    #[test]
    fn album_merge_preserves_omitted_fields() {
        let album = Album {
            id: "album-id".to_string(),
            name: "Original".to_string(),
            total_duration: 900,
            release_date: NaiveDate::from_ymd_opt(2024, 1, 1),
            artist_id: "artist-id".to_string(),
            image_path: Some("/images/album".to_string()),
            album_type: AlbumType::Ep,
        };

        let updated = album.merge_update(UpdateAlbum {
            name: Some("Updated".to_string()),
            release_date: None,
            artist_id: Some("new-artist-id".to_string()),
        });

        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.release_date, NaiveDate::from_ymd_opt(2024, 1, 1));
        assert_eq!(updated.artist_id, "new-artist-id");
        assert_eq!(updated.total_duration, 900);
        assert_eq!(updated.image_path.as_deref(), Some("/images/album"));
        assert_eq!(updated.album_type, AlbumType::Ep);
        assert!(
            !UpdateAlbum {
                name: None,
                release_date: None,
                artist_id: None,
            }
            .has_changes()
        );
    }
}
