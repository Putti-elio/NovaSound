//! Parse only the fields needed for music rankings. Never extract ZIP members to disk.
use std::io::{Cursor, Read};

use chrono::{DateTime, Utc};
use novasound_domain::top_artists::{
    ImportedHistory, ListeningEvent, Provider, RankedArtist, TopArtistsError, UserId,
    supported_timestamp,
};
use serde::{
    Deserialize,
    de::{DeserializeSeed, SeqAccess, Visitor},
};
use sha2::{Digest, Sha256};

pub const MAX_INPUT_BYTES: usize = 32 * 1024 * 1024;
const MAX_EXPANDED_BYTES: usize = 128 * 1024 * 1024;
const MAX_EVENTS: usize = 250_000;
const MAX_MEMBERS: usize = 1_000;

#[derive(Clone, Copy)]
pub enum HistoryFormat {
    Json,
    Zip,
}

/// CPU/blocking work: asynchronous callers must run this on a blocking worker.
pub fn parse_history(
    user_id: UserId,
    input: &[u8],
    format: HistoryFormat,
) -> Result<ImportedHistory, TopArtistsError> {
    if input.len() > MAX_INPUT_BYTES {
        return Err(TopArtistsError::ImportTooLarge);
    }
    let mut history = ImportedHistory {
        user_id,
        provider: Provider::Spotify,
        events: Vec::new(),
        skipped: 0,
    };
    match format {
        | HistoryFormat::Json => append_json(&mut history, input)?,
        | HistoryFormat::Zip => {
            let mut archive = zip::ZipArchive::new(Cursor::new(input))
                .map_err(|_| TopArtistsError::InvalidImport)?;
            if archive.len() > MAX_MEMBERS {
                return Err(TopArtistsError::ImportTooLarge);
            }
            let mut expanded = 0;
            let mut matched = false;
            for index in 0..archive.len() {
                let mut file = archive
                    .by_index(index)
                    .map_err(|_| TopArtistsError::InvalidImport)?;
                let path = file.enclosed_name().ok_or(TopArtistsError::InvalidImport)?;
                let name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .ok_or(TopArtistsError::InvalidImport)?;
                if file.is_dir()
                    || !name.ends_with(".json")
                    || !(name.starts_with("Streaming_History_Audio_")
                        || name.starts_with("endsong_"))
                {
                    continue;
                }
                matched = true;
                let remaining = MAX_EXPANDED_BYTES - expanded;
                if file.size() > remaining as u64 {
                    return Err(TopArtistsError::ImportTooLarge);
                }
                let mut bytes = Vec::new();
                (&mut file)
                    .take(remaining as u64 + 1)
                    .read_to_end(&mut bytes)
                    .map_err(|_| TopArtistsError::InvalidImport)?;
                if bytes.len() > remaining {
                    return Err(TopArtistsError::ImportTooLarge);
                }
                expanded += bytes.len();
                append_json(&mut history, &bytes)?;
            }
            if !matched {
                return Err(TopArtistsError::InvalidImport);
            }
        },
    }
    Ok(history)
}

#[derive(Deserialize)]
struct HistoryRow {
    ts: DateTime<Utc>,
    ms_played: i64,
    master_metadata_track_name: Option<String>,
    master_metadata_album_artist_name: Option<String>,
    master_metadata_album_album_name: Option<String>,
    spotify_track_uri: Option<String>,
}

fn append_json(history: &mut ImportedHistory, bytes: &[u8]) -> Result<(), TopArtistsError> {
    struct Rows<'a> {
        history: &'a mut ImportedHistory,
        failure: &'a mut Option<TopArtistsError>,
    }

    impl<'de> DeserializeSeed<'de> for Rows<'_> {
        type Value = ();

        fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
            struct RowsVisitor<'a>(Rows<'a>);

            impl<'de> Visitor<'de> for RowsVisitor<'_> {
                type Value = ();

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("an array of streaming history rows")
                }

                fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
                    while let Some(row) = seq.next_element::<HistoryRow>()? {
                        if self.0.history.events.len() as u64 + self.0.history.skipped
                            >= MAX_EVENTS as u64
                        {
                            *self.0.failure = Some(TopArtistsError::ImportTooLarge);
                            return Err(serde::de::Error::custom("event limit exceeded"));
                        }
                        if let Err(error) = append_row(self.0.history, row) {
                            *self.0.failure = Some(error);
                            return Err(serde::de::Error::custom("invalid history row"));
                        }
                    }
                    Ok(())
                }
            }

            deserializer.deserialize_seq(RowsVisitor(self))
        }
    }

    let mut failure = None;
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    Rows {
        history,
        failure: &mut failure,
    }
    .deserialize(&mut deserializer)
    .and_then(|()| deserializer.end())
    .map_err(|_| failure.unwrap_or(TopArtistsError::InvalidImport))
}

fn append_row(history: &mut ImportedHistory, row: HistoryRow) -> Result<(), TopArtistsError> {
    if row.ms_played < 0 || row.ms_played > i64::from(i32::MAX) || !supported_timestamp(row.ts) {
        return Err(TopArtistsError::InvalidImport);
    }
    let Some(name) = row.master_metadata_album_artist_name else {
        history.skipped += 1;
        return Ok(());
    };
    let track_name = row.master_metadata_track_name;
    let uri = row.spotify_track_uri;
    if track_name.is_none() && uri.is_none() {
        history.skipped += 1;
        return Ok(());
    }
    if name.trim().is_empty()
        || name.len() > 4096
        || track_name
            .as_ref()
            .is_some_and(|track| track.trim().is_empty() || track.len() > 4096)
    {
        return Err(TopArtistsError::InvalidImport);
    }
    if uri
        .as_ref()
        .is_some_and(|uri| !uri.starts_with("spotify:track:") || uri.len() > 256 || uri.len() == 14)
    {
        return Err(TopArtistsError::InvalidImport);
    }
    // Provider track IDs survive metadata corrections between exports. Only local music
    // needs a name-based content identity. No account/device/IP data is retained.
    let content_identity = match uri {
        | Some(uri) => serde_json::json!({"uri": uri}),
        | None => {
            serde_json::json!({"artist": name, "track": track_name, "album": row.master_metadata_album_album_name})
        },
    };
    let identity = serde_json::to_vec(&(row.ts, content_identity, row.ms_played))
        .map_err(|_| TopArtistsError::InvalidImport)?;
    history.events.push(ListeningEvent {
        fingerprint: format!("{:x}", Sha256::digest(identity)),
        artist: RankedArtist::from_provider(history.user_id, Provider::Spotify, None, name, None),
        ended_at: row.ts,
        played_ms: row.ms_played,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use uuid::Uuid;

    fn user() -> UserId {
        UserId::new(Uuid::from_u128(1)).unwrap()
    }
    pub(super) fn fixture() -> &'static [u8] {
        br#"[{"ts":"2026-09-01T12:00:00Z","ms_played":30000,"master_metadata_track_name":"Track","master_metadata_album_artist_name":"Artist","spotify_track_uri":"spotify:track:abc","ip_addr":"never persist"}]"#
    }

    #[test]
    fn normalizes_only_music_and_discards_sensitive_fields() {
        let history = parse_history(user(), fixture(), HistoryFormat::Json).unwrap();
        assert_eq!(history.events.len(), 1);
        assert_eq!(history.events[0].played_ms, 30000);
        assert_eq!(history.events[0].artist.providers[0].artist_id, None);
        assert!(!format!("{history:?}").contains("never persist"));
        let podcasts =
            br#"[{"ts":"2026-09-01T12:00:00Z","ms_played":30000,"episode_name":"Podcast"}]"#;
        assert_eq!(
            parse_history(user(), podcasts, HistoryFormat::Json)
                .unwrap()
                .skipped,
            1
        );
    }

    #[test]
    fn validates_input_and_canonicalizes_duplicates() {
        for input in [
            b"{}".as_slice(),
            b"[{}]",
            b"not json",
            br#"[{"ts":"bad","ms_played":0}]"#,
            br#"[{"ts":"2026-01-01T00:00:00Z","ms_played":-1}]"#,
        ] {
            assert!(matches!(
                parse_history(user(), input, HistoryFormat::Json),
                Err(TopArtistsError::InvalidImport)
            ));
        }
        let original = parse_history(user(), fixture(), HistoryFormat::Json).unwrap();
        let offset = String::from_utf8(fixture().to_vec())
            .unwrap()
            .replace("12:00:00Z", "14:00:00+02:00");
        let same = parse_history(user(), offset.as_bytes(), HistoryFormat::Json).unwrap();
        assert_eq!(original.events[0].fingerprint, same.events[0].fingerprint);
        let renamed = String::from_utf8(fixture().to_vec())
            .unwrap()
            .replace("\"Track\"", "\"Renamed track\"");
        let renamed = parse_history(user(), renamed.as_bytes(), HistoryFormat::Json).unwrap();
        assert_eq!(
            original.events[0].fingerprint,
            renamed.events[0].fingerprint
        );
        assert!(matches!(
            parse_history(user(), &vec![0; MAX_INPUT_BYTES + 1], HistoryFormat::Json),
            Err(TopArtistsError::ImportTooLarge)
        ));
    }

    #[test]
    fn imports_artist_with_track_uri_even_when_title_is_missing() {
        let input = br#"[{"ts":"2026-09-01T12:00:00Z","ms_played":30000,"master_metadata_album_artist_name":"Artist","spotify_track_uri":"spotify:track:abc"}]"#;
        let history = parse_history(user(), input, HistoryFormat::Json).unwrap();
        assert_eq!(history.events.len(), 1);
        assert_eq!(history.skipped, 0);
        let no_identity = br#"[{"ts":"2026-09-01T12:00:00Z","ms_played":30000,"master_metadata_album_artist_name":"Artist"}]"#;
        assert_eq!(
            parse_history(user(), no_identity, HistoryFormat::Json)
                .unwrap()
                .skipped,
            1
        );
    }

    #[test]
    fn rejects_too_many_rows_before_deserializing_trailing_input() {
        let row = br#"{"ts":"2026-09-01T00:00:00Z","ms_played":0,"master_metadata_album_artist_name":"A","spotify_track_uri":"spotify:track:a"}"#;
        let mut input = Vec::with_capacity((row.len() + 1) * (MAX_EVENTS + 1) + 2);
        input.push(b'[');
        for index in 0..=MAX_EVENTS {
            if index != 0 {
                input.push(b',');
            }
            input.extend_from_slice(row);
        }
        input.push(b']');
        assert!(input.len() <= MAX_INPUT_BYTES);
        assert!(matches!(
            parse_history(user(), &input, HistoryFormat::Json),
            Err(TopArtistsError::ImportTooLarge)
        ));

        // An eager Vec<HistoryRow> would parse this invalid suffix before checking
        // the row count, yielding InvalidImport instead of ImportTooLarge.
        input.pop();
        input.extend_from_slice(b",not-json]");
        assert!(matches!(
            parse_history(user(), &input, HistoryFormat::Json),
            Err(TopArtistsError::ImportTooLarge)
        ));
    }

    #[test]
    fn imports_zip_members_without_reading_unrelated_exports() {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default();
        writer
            .start_file("Spotify/Streaming_History_Audio_2026_0.json", options)
            .unwrap();
        writer.write_all(fixture()).unwrap();
        writer.start_file("endsong_0.json", options).unwrap();
        writer.write_all(fixture()).unwrap();
        writer
            .start_file("Technical_Log_Information.json", options)
            .unwrap();
        writer.write_all(b"secret, not history").unwrap();
        let archive = writer.finish().unwrap().into_inner();
        let history = parse_history(user(), &archive, HistoryFormat::Zip).unwrap();
        assert_eq!(history.events.len(), 2);
        assert_eq!(history.events[0].fingerprint, history.events[1].fingerprint);
        assert!(parse_history(user(), b"PK bad", HistoryFormat::Zip).is_err());
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        writer.start_file("../endsong_0.json", options).unwrap();
        writer.write_all(fixture()).unwrap();
        assert!(
            parse_history(
                user(),
                &writer.finish().unwrap().into_inner(),
                HistoryFormat::Zip
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_zip_expansion_member_limits_and_archives_without_history() {
        let options = zip::write::SimpleFileOptions::default();
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        writer.start_file("endsong_0.json", options).unwrap();
        std::io::copy(
            &mut std::io::repeat(b' ').take(MAX_EXPANDED_BYTES as u64 + 1),
            &mut writer,
        )
        .unwrap();
        let archive = writer.finish().unwrap().into_inner();
        assert!(archive.len() < MAX_INPUT_BYTES);
        assert!(matches!(
            parse_history(user(), &archive, HistoryFormat::Zip),
            Err(TopArtistsError::ImportTooLarge)
        ));

        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for index in 0..=MAX_MEMBERS {
            writer.start_file(format!("{index}.txt"), options).unwrap();
        }
        let archive = writer.finish().unwrap().into_inner();
        assert!(matches!(
            parse_history(user(), &archive, HistoryFormat::Zip),
            Err(TopArtistsError::ImportTooLarge)
        ));
        let archive = zip::ZipWriter::new(Cursor::new(Vec::new()))
            .finish()
            .unwrap()
            .into_inner();
        assert!(matches!(
            parse_history(user(), &archive, HistoryFormat::Zip),
            Err(TopArtistsError::InvalidImport)
        ));
    }

    #[test]
    fn local_music_has_name_identity_and_valid_empty_history_is_accepted() {
        let input = br#"[{"ts":"2026-09-01T00:00:00Z","ms_played":0,"master_metadata_track_name":"Local","master_metadata_album_artist_name":"Artist","spotify_track_uri":null}]"#;
        let history = parse_history(user(), input, HistoryFormat::Json).unwrap();
        assert_eq!(history.events.len(), 1);
        assert_eq!(history.events[0].played_ms, 0);
        assert!(history.events[0].artist.providers[0].artist_id.is_none());
        assert!(
            parse_history(user(), b"[]", HistoryFormat::Json)
                .unwrap()
                .events
                .is_empty()
        );
    }
}
