const DATE_FORMAT: &str = "%d-%m-%Y";

pub mod option_naive_date_dmy {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::models::date_serde::DATE_FORMAT;

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            | Some(value) => serializer.serialize_str(&value.format(DATE_FORMAT).to_string()),
            | None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<String>::deserialize(deserializer)?;

        match value {
            | Some(raw_date) => NaiveDate::parse_from_str(&raw_date, DATE_FORMAT)
                .map(Some)
                .map_err(|_| {
                    serde::de::Error::custom("Invalid date format. Expected format is DD-MM-YYYY")
                }),
            | None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct DateWrapper {
        #[serde(with = "crate::models::date_serde::option_naive_date_dmy")]
        release_date: Option<NaiveDate>,
    }

    #[test]
    fn serialize_uses_dd_mm_yyyy_format() {
        let value = DateWrapper {
            release_date: NaiveDate::from_ymd_opt(2025, 12, 31),
        };

        let json = serde_json::to_string(&value).expect("serialize date wrapper");

        assert_eq!(json, r#"{"release_date":"31-12-2025"}"#);
    }

    #[test]
    fn deserialize_accepts_dd_mm_yyyy_format() {
        let payload = r#"{"release_date":"01-01-2026"}"#;

        let parsed: DateWrapper = serde_json::from_str(payload).expect("deserialize date wrapper");

        assert_eq!(parsed.release_date, NaiveDate::from_ymd_opt(2026, 1, 1));
    }

    #[test]
    fn deserialize_rejects_timestamp_format() {
        let payload = r#"{"release_date":1704067200}"#;

        let result = serde_json::from_str::<DateWrapper>(payload);

        assert!(result.is_err());
    }
}
