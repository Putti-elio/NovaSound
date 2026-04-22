use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::song_model::AlbumType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub total_duration: u32,
    #[serde(default, with = "crate::models::date_serde::option_naive_date_dmy")]
    pub release_date: Option<NaiveDate>,
    pub artist_id: String,
    pub image_path: Option<String>,
    pub album_type: AlbumType,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlbum {
    pub name: String,
    #[serde(default, with = "crate::models::date_serde::option_naive_date_dmy")]
    pub release_date: Option<NaiveDate>,
    pub artist_id: String,
    pub album_type: Option<AlbumType>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAlbum {
    pub name: Option<String>,
    #[serde(default, with = "crate::models::date_serde::option_naive_date_dmy")]
    pub release_date: Option<NaiveDate>,
    pub artist_id: Option<String>,
}
