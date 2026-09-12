use chrono::NaiveDate;

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
