use serde::Deserialize;


#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    port: Option<u16>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}
