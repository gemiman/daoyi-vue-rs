use crate::configs::AppConfig;
use sa_token_plugin_axum::{LoggingListener, StpUtil, TokenStyle};
use std::sync::Arc;
use axum::http::header;

pub async fn init() -> anyhow::Result<()> {
    let redis_config = AppConfig::get().await.redis();
    let config = sa_token_storage_redis::RedisConfig {
        host: String::from(redis_config.host()),
        port: redis_config.port(),
        password: Some(String::from(redis_config.password())),
        database: redis_config.database(),
        pool_size: 10,
    };

    let storage = sa_token_storage_redis::RedisStorage::from_config(config, "sa-token:").await?;

    let _state = sa_token_plugin_axum::SaTokenState::builder()
        .token_style(TokenStyle::SimpleUuid)
        .storage(Arc::new(storage))
        .timeout(86400)
        .build();
    StpUtil::register_listener(Arc::new(LoggingListener));
    Ok(())
}
