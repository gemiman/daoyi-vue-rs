use crate::configs::AppConfig;
pub use tracing as log;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub async fn init() {
    let log_config = AppConfig::get().await.log();

    // 根据配置创建文件 appender
    let file_appender = match log_config.rolling() {
        "hourly" => rolling::hourly(log_config.dir(), log_config.filename()),
        "minutely" => rolling::minutely(log_config.dir(), log_config.filename()),
        "never" => rolling::never(log_config.dir(), log_config.filename()),
        _ => rolling::daily(log_config.dir(), log_config.filename()), // 默认按天分割
    };

    // 创建非阻塞写入器
    let (non_blocking_file, _guard) = non_blocking(file_appender);

    // 控制台输出层
    let console_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%Y-%m-%d %H:%M:%S%.3f".to_string(),
        ))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false);

    // 文件输出层
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_file)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%Y-%m-%d %H:%M:%S%.3f".to_string(),
        ))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .with_ansi(false); // 文件输出不需要颜色代码

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(log_config.level())),
        )
        .with(console_layer)
        .with(file_layer)
        .init();

    // 注意：_guard 需要保持存活，否则日志会丢失
    // 可以考虑将其存储在全局变量中
    std::mem::forget(_guard);
}
