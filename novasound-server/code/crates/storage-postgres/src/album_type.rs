use anyhow::{Result, bail};
use novasound_domain::models::song_model::AlbumType;

#[must_use]
pub fn to_database_value(album_type: &AlbumType) -> &'static str {
    match album_type {
        | AlbumType::Album => "ALBUM",
        | AlbumType::Ep => "EP",
        | AlbumType::Single => "SINGLE",
        | AlbumType::StandaloneCollection => "STANDALONE_COLLECTION",
    }
}

pub fn from_database_value(value: &str) -> Result<AlbumType> {
    match value {
        | "ALBUM" => Ok(AlbumType::Album),
        | "EP" => Ok(AlbumType::Ep),
        | "SINGLE" => Ok(AlbumType::Single),
        | "STANDALONE_COLLECTION" => Ok(AlbumType::StandaloneCollection),
        | _ => bail!("unknown PostgreSQL album_type value '{value}'"),
    }
}

#[cfg(test)]
mod tests {
    use super::{from_database_value, to_database_value};
    use novasound_domain::models::song_model::AlbumType;

    #[test]
    fn maps_album_types_to_postgresql_values() {
        assert_eq!(to_database_value(&AlbumType::Ep), "EP");
    }

    #[test]
    fn rejects_unknown_postgresql_values() {
        let error = from_database_value("MIXTAPE").expect_err("unknown value must fail");

        assert!(error.to_string().contains("MIXTAPE"));
    }
}
