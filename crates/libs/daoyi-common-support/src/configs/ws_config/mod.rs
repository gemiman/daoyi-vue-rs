use merge::Merge;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Merge)]
pub struct WsConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    enable: Option<bool>,
}

impl WsConfig {
    pub fn enable(&self) -> bool {
        self.enable.unwrap_or(true)
    }
}
