use serde::{Deserialize, Deserializer};

/// 灵活的字符串类型，可以从字符串或数字反序列化
/// 可以直接当作 String 使用，无需手动转换
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FlexString(pub String);

impl<'de> Deserialize<'de> for FlexString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrNumber {
            String(String),
            Number(i64),
            Float(f64),
            Bool(bool),
        }

        match StringOrNumber::deserialize(deserializer)? {
            StringOrNumber::String(s) => Ok(FlexString(s)),
            StringOrNumber::Number(n) => Ok(FlexString(n.to_string())),
            StringOrNumber::Float(f) => Ok(FlexString(f.to_string())),
            StringOrNumber::Bool(b) => Ok(FlexString(b.to_string())),
        }
    }
}

impl serde::Serialize for FlexString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// 自动转换为 String
impl From<FlexString> for String {
    fn from(val: FlexString) -> Self {
        val.0
    }
}

impl From<String> for FlexString {
    fn from(s: String) -> Self {
        FlexString(s)
    }
}

impl From<&str> for FlexString {
    fn from(s: &str) -> Self {
        FlexString(s.to_string())
    }
}

// 可以像 String 一样使用
impl AsRef<str> for FlexString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for FlexString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for FlexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for FlexString {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<String> for FlexString {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

/// 可选的灵活字符串类型，支持 null、字符串、数字
pub type OptFlexString = Option<FlexString>;
