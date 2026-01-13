use crate::configs::AppConfig;
use crate::enumeration::WEBSOCKET_REDIS_CHANNEL;
use crate::websocket::{RedisWebSocketMessage, WebSocketMessageSender, WebSocketSessionManager};
use crate::{database, id_util, logger, redis_utils, server};
use axum::Router;
use futures::StreamExt;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub ws_manager: Arc<WebSocketSessionManager>,
    pub ws_sender: Arc<WebSocketMessageSender>,
}

impl AppState {
    pub fn new() -> Self {
        let ws_manager = Arc::new(WebSocketSessionManager::new());
        let ws_sender = Arc::new(WebSocketMessageSender::new(ws_manager.clone()));
        Self {
            ws_manager,
            ws_sender,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn run(app_name: Option<&str>, router: Router<AppState>) -> anyhow::Result<()> {
    println!("==============================================开始加载配置...");
    AppConfig::load(app_name.unwrap_or("app")).await?;
    println!("==============================================配置加载完成...开始初始化日志组件....");
    logger::init_logger().await;
    tracing::info!("日志组件初始化完成... Starting redis_utils...");
    redis_utils::init_redis().await?;
    tracing::info!("redis组件初始化完成... Starting id generator...");
    id_util::init().await?;
    tracing::info!("id generator 初始化完成... Starting database...");
    database::init_db().await?;

    tracing::info!("database 初始化完成... Starting app server...");
    let state = AppState::new();

    // 启动 Redis WebSocket 订阅任务
    start_redis_ws_subscription(state.ws_sender.clone()).await;

    let server = server::Server::new(AppConfig::get().server());
    server.start(state, router).await
}

async fn start_redis_ws_subscription(ws_sender: Arc<WebSocketMessageSender>) {
    tokio::spawn(async move {
        let redis_config = AppConfig::get().redis();
        let host = redis_config.host();
        let port = redis_config.port();
        let db = redis_config.database();
        let passwd = redis_config.password();

        let url = if passwd.is_empty() {
            format!("redis://{host}:{port}/{db}")
        } else {
            format!("redis://:{passwd}@{host}:{port}/{db}")
        };

        let client = match deadpool_redis::redis::Client::open(url) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to create redis client for subscription: {}", e);
                return;
            }
        };

        loop {
            match client.get_async_pubsub().await {
                Ok(mut pub_sub) => {
                    if let Err(e) = pub_sub.subscribe(WEBSOCKET_REDIS_CHANNEL).await {
                        tracing::error!(
                            "Failed to subscribe to redis channel {}: {}",
                            WEBSOCKET_REDIS_CHANNEL,
                            e
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        continue;
                    }

                    tracing::info!("Subscribed to redis channel: {}", WEBSOCKET_REDIS_CHANNEL);
                    let mut stream = pub_sub.on_message();
                    while let Some(msg) = stream.next().await {
                        // 修正 payload 获取逻辑
                        let payload_res: deadpool_redis::redis::RedisResult<String> =
                            msg.get_payload();
                        if let Ok(payload) = payload_res {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<RedisWebSocketMessage>(&payload)
                            {
                                ws_sender.send_local(ws_msg).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Redis connection failed for subscription: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    });
}
