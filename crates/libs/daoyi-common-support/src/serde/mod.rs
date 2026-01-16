use crate::error::{ApiError, ApiResult};
use sea_orm::prelude::Json;
use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;
use validator::Validate;

pub fn de_comma_separated<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    s.split(',')
        .map(|v| v.trim().parse().map_err(serde::de::Error::custom))
        .collect()
}
#[derive(Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber<T> {
    String(String),
    Number(T),
}
pub fn deserialize_numer<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: Display,
    D: Deserializer<'de>,
{
    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
/// Option<DateTime> 的序列化/反序列化支持
pub mod option_datetime_format {
    use crate::serde::FORMAT;
    use sea_orm::prelude::DateTime;
    use sea_orm::sqlx::types::chrono;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.collect_str(&dt.format(FORMAT)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrTimestamp {
            String(String),
            Timestamp(i64),
        }

        let opt: Option<StringOrTimestamp> = Option::deserialize(deserializer)?;
        match opt {
            Some(StringOrTimestamp::String(s)) => {
                let dt = DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                Ok(Some(dt))
            }
            Some(StringOrTimestamp::Timestamp(ts)) => {
                let dt = chrono::DateTime::from_timestamp_millis(ts)
                    .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?
                    .naive_local();
                Ok(Some(dt))
            }
            None => Ok(None),
        }
    }
}

pub mod datetime_format {
    use crate::serde::FORMAT;
    use sea_orm::prelude::DateTime;
    use sea_orm::sqlx::types::chrono;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&date.format(FORMAT))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrTimestamp {
            String(String),
            Timestamp(i64),
        }

        match StringOrTimestamp::deserialize(deserializer)? {
            StringOrTimestamp::String(s) => {
                DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
            }
            StringOrTimestamp::Timestamp(ts) => Ok(chrono::DateTime::from_timestamp_millis(ts)
                .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?
                .naive_local()),
        }
        // let s = String::deserialize(deserializer)?;
        // DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod option_vec_datetime_format {
    use sea_orm::prelude::DateTime;
    use serde::{Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<DateTime>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Option<Vec<String>> = Option::deserialize(deserializer)?;
        match v {
            Some(vec) => {
                let mut dates = Vec::new();
                for s in vec {
                    let date =
                        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                    dates.push(date);
                }
                Ok(Some(dates))
            }
            None => Ok(None),
        }
    }
}

pub fn validate_and_parse<T>(config: &Json) -> ApiResult<T>
where
    T: serde::de::DeserializeOwned + Validate,
{
    let parsed: T = T::deserialize(config).map_err(serde_json::Error::from)?;
    parsed
        .validate()
        .map_err(|e| ApiError::valid(e.to_string()))?;
    Ok(parsed)
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
    }
}
