use crate::configs::AppConfig;
use crate::{database, id_util, logger, redis_utils, server};
use axum::Router;
use tracing::log;

#[derive(Clone)]
pub struct AppState {}

pub async fn run(app_name: Option<&str>, router: Router<AppState>) -> anyhow::Result<()> {
    println!("==============================================开始加载配置...");
    AppConfig::load(app_name.unwrap_or("app")).await?;
    // println!("配置信息：\n{:#?}", AppConfig::get());
    println!("==============================================配置加载完成...开始初始化日志组件....");
    logger::init_logger().await;
    log::info!("日志组件初始化完成... Starting redis_utils...");
    redis_utils::init_redis().await?;
    log::info!("redis组件初始化完成... Starting id generator...");
    id_util::init().await?;
    log::info!("id generator 初始化完成... Starting database...");
    database::init_db().await?;
    log::info!("database 初始化完成... Starting app server...");
    let state = AppState {};
    let server = server::Server::new(AppConfig::get().server());
    server.start(state, router).await
}
