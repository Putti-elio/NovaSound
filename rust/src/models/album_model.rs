use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::song_model::AlbumType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub total_duration: u32,
    #[serde(with = "date_serde")]
    pub release_date: Option<NaiveDate>,
    pub artist_id: String,
    pub image_path: Option<String>,
    pub album_type: AlbumType,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlbum {
    pub name: String,
    pub release_date: Option<NaiveDate>,
    pub artist_id: String,
    pub album_type: Option<AlbumType>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAlbum {
    pub name: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub artist_id: Option<String>,
}

mod date_serde {
    use chrono::{NaiveDate, NaiveTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                let datetime = d
                    .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                    .and_utc();
                serializer.serialize_i64(datetime.timestamp())
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp: Option<i64> = Option::deserialize(deserializer)?;
        match timestamp {
            Some(ts) => {
                let datetime = chrono::DateTime::from_timestamp(ts, 0)
                    .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?;
                Ok(Some(datetime.date_naive()))
            }
            None => Ok(None),
        }
    }
}
