pub const MAIL_SEND_STREAM_KEY: &str = "system:mail:send:stream";
pub const MAIL_SEND_GROUP_NAME: &str = "system:mail:send:group";
pub const CHANNEL_OPERATE_LOG: &str = "system:operate:log";

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RedisKey {
    CheckToken,
    CheckTenantId,
    RoleById,
}

impl RedisKey {
    pub fn key<M: AsRef<str> + std::fmt::Display>(&self, key: M) -> String {
        format!("{}:{}", self, key)
    }
}
