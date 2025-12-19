pub mod server_config;
mod log_config;

use anyhow::{Context, anyhow};
use config::{Config, FileFormat};
use serde::Deserialize;
pub use server_config::ServerConfig;
use std::sync::LazyLock;
use tokio::sync::OnceCell;

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::const_new();
static DEFAULT_SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| ServerConfig::default());

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: Option<ServerConfig>,
}

impl AppConfig {
    pub fn server(&self) -> &ServerConfig {
        self.server.as_ref().unwrap_or(&DEFAULT_SERVER_CONFIG)
    }
    async fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow!("Failed to load application config"))?
            .try_deserialize()
            .with_context(|| anyhow!("Failed to deserialize application config"))
    }

    pub async fn get() -> &'static Self {
        APP_CONFIG
            .get_or_init(|| async {
                AppConfig::load()
                    .await
                    .expect("Failed to load application config")
            })
            .await
    }
}
