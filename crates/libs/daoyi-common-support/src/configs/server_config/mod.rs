use bytesize::ByteSize;
use merge::Merge;
use serde::Deserialize;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Deserialize, Default, Merge)]
pub struct ServerConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    port: Option<u16>,
    #[merge(strategy = merge::option::overwrite_none)]
    timeout: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    max_body_size: Option<String>,
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
        if let Some(max_body_size) = &self.max_body_size {
            return ByteSize::from_str(max_body_size)
                .unwrap_or(ByteSize::mib(10))
                .as_u64() as usize;
        }
        ByteSize::mib(10).as_u64() as usize
    }

    pub fn max_age(&self) -> Duration {
        if let Some(max_age) = &self.max_age {
            return humantime::parse_duration(max_age).unwrap_or(Duration::from_secs(3600 * 12));
        }
        Duration::from_secs(3600 * 12)
    }
}
