use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct LogConfig {
    level: Option<String>,
    dir: Option<String>,
    filename: Option<String>,
    rolling: Option<String>,
}

impl LogConfig {
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
