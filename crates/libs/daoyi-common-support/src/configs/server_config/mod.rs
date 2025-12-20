use merge::Merge;
use serde::Deserialize;


#[derive(Debug, Deserialize, Default, Merge)]
pub struct ServerConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    port: Option<u16>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}
