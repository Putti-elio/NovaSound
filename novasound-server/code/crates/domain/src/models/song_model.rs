use chrono::NaiveDate;

use crate::rules::has_non_empty_name;
use crate::validation::{ValidationErrors, ValidationIssue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlbumType {
    Album,
    Ep,
    Single,
    StandaloneCollection,
}

#[derive(Debug)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub duration: u32,
    pub artist_id: String,
    pub album_id: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub track_number: Option<i32>,
    pub image_path: Option<String>,
}

#[derive(Debug)]
pub struct CreateSong {
    pub name: String,
    pub duration: u32,
    pub artist_id: String,
    pub album_id: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub track_number: Option<i32>,
}

#[derive(Debug)]
pub struct UpdateSong {
    pub name: Option<String>,
    pub duration: Option<u32>,
    pub release_date: Option<NaiveDate>,
    pub track_number: Option<i32>,
}

impl UpdateSong {
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.name.is_some()
            || self.duration.is_some()
            || self.release_date.is_some()
            || self.track_number.is_some()
    }

    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self
            .name
            .as_deref()
            .is_some_and(|name| !has_non_empty_name(name))
        {
            errors.push(ValidationIssue::new(
                "name",
                "required",
                "Song name cannot be empty",
            ));
        }
        if self
            .duration
            .is_some_and(|duration| i32::try_from(duration).is_err())
        {
            errors.push(ValidationIssue::new(
                "duration",
                "out_of_range",
                "Song duration is too large",
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl CreateSong {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if !has_non_empty_name(&self.name) {
            errors.push(ValidationIssue::new(
                "name",
                "required",
                "Song name cannot be empty",
            ));
        }
        if i32::try_from(self.duration).is_err() {
            errors.push(ValidationIssue::new(
                "duration",
                "out_of_range",
                "Song duration is too large",
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Song {
    #[must_use]
    pub fn merge_update(self, update: UpdateSong) -> Self {
        Self {
            id: self.id,
            name: update.name.unwrap_or(self.name),
            duration: update.duration.unwrap_or(self.duration),
            artist_id: self.artist_id,
            album_id: self.album_id,
            release_date: update.release_date.or(self.release_date),
            track_number: update.track_number.or(self.track_number),
            image_path: self.image_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CreateSong;

    #[test]
    fn create_validation_collects_existing_song_failures() {
        let errors = CreateSong {
            name: " \t".to_string(),
            duration: (i32::MAX as u32) + 1,
            artist_id: "artist-id".to_string(),
            album_id: None,
            release_date: None,
            track_number: None,
        }
        .validate()
        .expect_err("invalid song command");

        assert_eq!(errors.issues().len(), 2);
        assert_eq!(errors.issues()[0].field, "name");
        assert_eq!(errors.issues()[0].code, "required");
        assert_eq!(errors.issues()[1].field, "duration");
        assert_eq!(errors.issues()[1].code, "out_of_range");
    }
}
