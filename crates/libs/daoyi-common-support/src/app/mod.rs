use crate::auth::sa_token_auth;
use crate::configs::AppConfig;
use crate::{database, id, logger, redis_utils, server};
use axum::Router;
use tracing::log;

#[derive(Clone, Default)]
pub struct AppState {}

pub async fn run(app_name: Option<&str>, router: Router<AppState>) -> anyhow::Result<()> {
    println!("==============================================开始加载配置...");
    AppConfig::load(app_name.unwrap_or("app")).await?;
    println!("配置信息：\n{:#?}", AppConfig::get().await);
    println!("==============================================配置加载完成...开始初始化日志组件....");
    logger::init().await;
    log::info!("日志组件初始化完成... Starting redis_utils...");
    redis_utils::init_redis().await?;
    log::info!("redis组件初始化完成... Starting id generator...");
    id::init().await?;
    log::info!("id generator 初始化完成... Starting database...");
    database::init().await?;
    log::info!("database 初始化完成... Starting sa token...");
    sa_token_auth::init().await?;
    log::info!("sa token 初始化完成... Starting app server...");
    let state = AppState::default();
    let server = server::Server::new(AppConfig::get().await.server());
    server.start(state, router).await
}
