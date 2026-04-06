use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AlbumType {
    Album,
    Ep,
    Single,
    StandaloneCollection,
}

impl AlbumType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlbumType::Album => "ALBUM",
            AlbumType::Ep => "EP",
            AlbumType::Single => "SINGLE",
            AlbumType::StandaloneCollection => "STANDALONE_COLLECTION",
        }
    }

    pub fn from_str(s: &str) -> Option<AlbumType> {
        match s {
            "ALBUM" => Some(AlbumType::Album),
            "EP" => Some(AlbumType::Ep),
            "SINGLE" => Some(AlbumType::Single),
            "STANDALONE_COLLECTION" => Some(AlbumType::StandaloneCollection),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct CreateSong {
    pub name: String,
    pub duration: u32,
    pub artist_id: String,
    pub album_id: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub track_number: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSong {
    pub name: Option<String>,
    pub duration: Option<u32>,
    pub release_date: Option<NaiveDate>,
    pub track_number: Option<i32>,
}
