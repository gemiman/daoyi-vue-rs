use crate::configs::AppConfig;
use crate::{database, id, logger, server};
use axum::Router;
use tracing::log;

#[derive(Clone, Default)]
pub struct AppState {}

pub async fn run(app_name: Option<&str>, router: Router<AppState>) -> anyhow::Result<()> {
    AppConfig::load(app_name.unwrap_or("app")).await?;
    logger::init().await;
    log::info!("Starting app server...");
    id::init().await?;
    database::init().await?;
    let state = AppState::default();
    let server = server::Server::new(AppConfig::get().await.server());
    server.start(state, router).await
}
