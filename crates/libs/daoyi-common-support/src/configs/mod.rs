pub mod log_config;
pub mod server_config;
use anyhow::{Context, anyhow};
use config::{Config, FileFormat};
pub use log_config::LogConfig;
use merge::Merge;
use serde::Deserialize;
pub use server_config::ServerConfig;
use std::sync::LazyLock;
use tokio::sync::OnceCell;

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::const_new();
static DEFAULT_SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| ServerConfig::default());
static DEFAULT_LOG_CONFIG: LazyLock<LogConfig> = LazyLock::new(|| LogConfig::default());

#[derive(Debug, Deserialize, Merge, Default)]
pub struct AppConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    name: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    active_profile: Option<String>,
    #[merge(strategy = merge::option::recurse)] // 递归合并
    server: Option<ServerConfig>,
    #[merge(strategy = merge::option::recurse)]
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
        // 按顺序加载并深度合并配置文件
        let mut init_config = AppConfig::default();

        // 1. application.yaml
        init_config = merge_config(
            init_config,
            load_one_config_file(&format!("{}/application", config_dir)).await?,
        );

        // 2. application-{profile}.yaml
        init_config = merge_config(
            init_config,
            load_one_config_file(&format!("{}/application-{}", config_dir, active_profile)).await?,
        );

        // 3. {app_name}.yaml
        init_config = merge_config(
            init_config,
            load_one_config_file(&format!("{}/{}", config_dir, app_name)).await?,
        );

        // 4. {app_name}-{profile}.yaml
        init_config = merge_config(
            init_config,
            load_one_config_file(&format!("{}/{}-{}", config_dir, app_name, active_profile)).await?,
        );

        // 5. 环境变量配置
        if let Ok(env_config) = Config::builder()
            .add_source(
                config::Environment::with_prefix("DY")
                    .try_parsing(true)
                    .separator("__")
                    .list_separator(","),
            )
            .build()
            .and_then(|c| c.try_deserialize::<AppConfig>())
        {
            init_config = merge_config(
                init_config,
                Some(env_config),
            );
        }
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
    }
}

fn merge_config(left: AppConfig, right: Option<AppConfig>) -> AppConfig {
    if let Some(right) = right {
        let mut merged_config = right;
        merged_config.merge(left);
        merged_config
    } else {
        left
    }
}

async fn load_one_config_file(file_name: &str) -> anyhow::Result<Option<AppConfig>> {
    Config::builder()
        .add_source(
            config::File::with_name(file_name)
                .format(FileFormat::Yaml)
                .required(false),
        )
        .build()
        .with_context(|| anyhow!("Failed to load {file_name}.yaml"))?
        .try_deserialize()
        .with_context(|| anyhow!("Failed to deserialize {file_name}.yaml"))
}
