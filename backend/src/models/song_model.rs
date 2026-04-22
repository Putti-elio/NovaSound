use std::fmt;
use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AlbumType {
    Album,
    Ep,
    Single,
    StandaloneCollection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseAlbumTypeError;

impl fmt::Display for ParseAlbumTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid album type")
    }
}

impl std::error::Error for ParseAlbumTypeError {}

impl AsRef<str> for AlbumType {
    fn as_ref(&self) -> &str {
        match self {
            | AlbumType::Album => "ALBUM",
            | AlbumType::Ep => "EP",
            | AlbumType::Single => "SINGLE",
            | AlbumType::StandaloneCollection => "STANDALONE_COLLECTION",
        }
    }
}

impl fmt::Display for AlbumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for AlbumType {
    type Err = ParseAlbumTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            | "ALBUM" => Ok(AlbumType::Album),
            | "EP" => Ok(AlbumType::Ep),
            | "SINGLE" => Ok(AlbumType::Single),
            | "STANDALONE_COLLECTION" => Ok(AlbumType::StandaloneCollection),
            | _ => Err(ParseAlbumTypeError),
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
    #[serde(default, with = "crate::models::date_serde::option_naive_date_dmy")]
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
    #[serde(default, with = "crate::models::date_serde::option_naive_date_dmy")]
    pub release_date: Option<NaiveDate>,
    pub track_number: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSong {
    pub name: Option<String>,
    pub duration: Option<u32>,
    #[serde(default, with = "crate::models::date_serde::option_naive_date_dmy")]
    pub release_date: Option<NaiveDate>,
    pub track_number: Option<i32>,
}
