#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RedisKey {
    CheckToken,
}

impl RedisKey {
    pub fn key<M: AsRef<str> + std::fmt::Display>(&self, key: M) -> String {
        format!("{}:{}", self, key)
    }
}
