use bytesize::ByteSize;
use merge::Merge;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Default, Merge)]
pub struct ServerConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    port: Option<u16>,
    #[merge(strategy = merge::option::overwrite_none)]
    timeout: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    max_body_size: Option<usize>,
    #[merge(strategy = merge::option::overwrite_none)]
    max_age: Option<String>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
    pub fn timeout(&self) -> Duration {
        if let Some(timeout) = &self.timeout {
            return humantime::parse_duration(timeout).unwrap_or(Duration::from_secs(120));
        }
        Duration::from_secs(120)
    }

    pub fn max_body_size(&self) -> usize {
        ByteSize::mib(10).as_u64() as usize
    }

    pub fn max_age(&self) -> Duration {
        Duration::from_secs(3600 * 12)
    }
}
