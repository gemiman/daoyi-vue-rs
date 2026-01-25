use merge::Merge;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Merge)]
pub struct LogConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    level: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    dir: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    filename: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    rolling: Option<String>,
    #[merge(strategy = merge::option::overwrite_none)]
    enable_operate_log: Option<bool>,
    #[merge(strategy = merge::option::overwrite_none)]
    log_server_url: Option<String>,
}

impl LogConfig {
    pub fn tracing_level(&self) -> tracing::Level {
        self.level().parse().unwrap_or(tracing::Level::INFO)
    }
    pub fn level(&self) -> &str {
        self.level.as_deref().unwrap_or("info")
    }

    pub fn dir(&self) -> &str {
        self.dir.as_deref().unwrap_or("./logs")
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    /// 获取日志滚动策略: daily, hourly, minutely, never
    pub fn rolling(&self) -> &str {
        self.rolling.as_deref().unwrap_or("daily")
    }

    pub fn enable_operate_log(&self) -> bool {
        self.enable_operate_log.unwrap_or(false)
    }

    pub fn log_server_url(&self) -> &str {
        self.log_server_url
            .as_deref()
            .unwrap_or("http://127.0.0.1:48001/admin-api/system/operate-log/create")
    }
}
