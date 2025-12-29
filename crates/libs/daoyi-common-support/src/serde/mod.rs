use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T> {
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

pub mod datetime_format {
    use sea_orm::prelude::DateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
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
                    let date = DateTime::parse_from_str(&s, FORMAT)
                        .map_err(serde::de::Error::custom)?;
                    dates.push(date);
                }
                Ok(Some(dates))
            }
            None => Ok(None),
        }
    }
}
