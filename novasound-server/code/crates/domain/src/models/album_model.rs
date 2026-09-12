use chrono::NaiveDate;

use crate::models::song_model::AlbumType;
use crate::rules::has_non_empty_name;
use crate::validation::{ValidationErrors, ValidationIssue};

#[derive(Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub total_duration: u32,
    pub release_date: Option<NaiveDate>,
    pub artist_id: String,
    pub image_path: Option<String>,
    pub album_type: AlbumType,
}

#[derive(Debug)]
pub struct CreateAlbum {
    pub name: String,
    pub release_date: Option<NaiveDate>,
    pub artist_id: String,
    pub album_type: Option<AlbumType>,
}

#[derive(Debug)]
pub struct UpdateAlbum {
    pub name: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub artist_id: Option<String>,
}

impl UpdateAlbum {
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.name.is_some() || self.release_date.is_some() || self.artist_id.is_some()
    }

    pub fn validate(&self) -> Result<(), ValidationErrors> {
        if self.name.as_deref().is_none_or(has_non_empty_name) {
            Ok(())
        } else {
            Err(ValidationIssue::new("name", "required", "Album name cannot be empty").into())
        }
    }
}

impl CreateAlbum {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        if has_non_empty_name(&self.name) {
            Ok(())
        } else {
            Err(ValidationIssue::new("name", "required", "Album name cannot be empty").into())
        }
    }
}

impl Album {
    #[must_use]
    pub fn merge_update(self, update: UpdateAlbum) -> Self {
        Self {
            id: self.id,
            name: update.name.unwrap_or(self.name),
            total_duration: self.total_duration,
            release_date: update.release_date.or(self.release_date),
            artist_id: update.artist_id.unwrap_or(self.artist_id),
            image_path: self.image_path,
            album_type: self.album_type,
        }
    }
}
