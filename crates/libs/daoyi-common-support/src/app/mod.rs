use crate::configs::AppConfig;
use crate::websocket::{WebSocketMessageSender, WebSocketSessionManager};
use crate::{database, id_util, logger, mail_server, redis_utils, server, websocket};
use axum::Router;
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
    // println!("AppConfig: {:?}", AppConfig::get());
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

    if AppConfig::get().ws().enable() {
        // 启动 Redis WebSocket 订阅任务
        websocket::start_redis_ws_subscription().await?;
    }

    if AppConfig::get().mail_server().enable() {
        // 初始化邮件队列消费者
        mail_server::init_mail_queue_consumer().await?;
    }

    if AppConfig::get().log().enable_operate_log() {
        // 初始化日志队列消费者
        logger::init_operate_log_subscriber().await?;
    }

    let server = server::Server::new(AppConfig::get().server());
    server.start(state, router).await
}
