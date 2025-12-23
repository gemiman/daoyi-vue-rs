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
}
