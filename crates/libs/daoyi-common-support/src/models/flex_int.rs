use crate::serde::StringOrNumber;
use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

/// FlexibleInt - 一个可以自动兼容字符串和数字的整数类型
/// 使用示例：pub sort: FlexibleInt<i32>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlexibleInt<T>(pub T);

impl<T> FlexibleInt<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for FlexibleInt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<FlexibleInt<T>> for i32
where
    T: Into<i32>,
{
    fn from(value: FlexibleInt<T>) -> Self {
        value.0.into()
    }
}

impl<'de, T> Deserialize<'de> for FlexibleInt<T>
where
    T: FromStr + Deserialize<'de>,
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match StringOrNumber::deserialize(deserializer)? {
            StringOrNumber::String(s) => {
                let value = s.parse::<T>().map_err(serde::de::Error::custom)?;
                Ok(FlexibleInt(value))
            }
            StringOrNumber::Number(n) => Ok(FlexibleInt(n)),
        }
    }
}

impl<T: serde::Serialize> serde::Serialize for FlexibleInt<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T: Display> Display for FlexibleInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
