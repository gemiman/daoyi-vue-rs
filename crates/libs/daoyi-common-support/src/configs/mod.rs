pub mod log_config;
pub mod server_config;

use anyhow::{Context, anyhow};
use config::{Config, FileFormat};
pub use log_config::LogConfig;
use serde::Deserialize;
pub use server_config::ServerConfig;
use std::sync::LazyLock;
use tokio::sync::OnceCell;

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::const_new();
static DEFAULT_SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| ServerConfig::default());
static DEFAULT_LOG_CONFIG: LazyLock<LogConfig> = LazyLock::new(|| LogConfig::default());

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    name: Option<String>,
    active_profile: Option<String>,
    server: Option<ServerConfig>,
    log: Option<LogConfig>,
}

impl AppConfig {
    pub fn app_name(&self) -> &str {
        self.name.as_deref().unwrap_or("app")
    }
    pub fn active_profile(&self) -> &str {
        self.active_profile.as_deref().unwrap_or("dev")
    }
    pub fn server(&self) -> &ServerConfig {
        self.server.as_ref().unwrap_or(&DEFAULT_SERVER_CONFIG)
    }
    pub fn log(&self) -> &LogConfig {
        self.log.as_ref().unwrap_or(&DEFAULT_LOG_CONFIG)
    }
    pub async fn load(app_name: &str) -> anyhow::Result<()> {
        let app_config = APP_CONFIG.get();
        if app_config.is_some() {
            return Ok(());
        }
        // 从环境变量获取配置目录，默认为 resources
        let config_dir = std::env::var("CONFIG_DIR")
            .or_else(|_| std::env::var("DY_CONFIG_DIR"))
            .unwrap_or_else(|_| "resources".to_string());
        // 从环境变量获取配置目录，默认为 resources
        let active_profile = std::env::var("ACTIVE_PROFILE")
            .or_else(|_| std::env::var("DY_ACTIVE_PROFILE"))
            .unwrap_or_else(|_| "dev".to_string());
        let mut init_config: Self = Config::builder()
            .add_source(
                config::File::with_name(&format!("{}/application", config_dir))
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            .add_source(
                config::File::with_name(&format!("{}/application-{}", config_dir, active_profile))
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            .add_source(
                config::File::with_name(&format!("{}/{}", config_dir, app_name))
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            .add_source(
                config::File::with_name(&format!("{}/{}-{}", config_dir, app_name, active_profile))
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            .add_source(
                config::Environment::with_prefix("DY")
                    .try_parsing(true)
                    .separator("__")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow!("Failed to load application config"))?
            .try_deserialize()
            .with_context(|| anyhow!("Failed to deserialize application config"))?;
        init_config.name = Some(app_name.to_string());
        init_config.active_profile = Some(active_profile);
        APP_CONFIG
            .set(init_config)
            .with_context(|| anyhow!("Failed to set application config"))?;
        Ok(())
    }

    pub async fn get() -> &'static Self {
        APP_CONFIG
            .get()
            .unwrap_or_else(|| panic!("Failed to load application config"))
        // APP_CONFIG
        //     .get_or_init(|| async {
        //         AppConfig::load()
        //             .await
        //             .expect("Failed to load application config")
        //     })
        //     .await
    }
}
