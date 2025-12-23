use crate::configs::AppConfig;
use crate::{database, logger, server};
use axum::Router;
use sea_orm::DatabaseConnection;
use tracing::log;

#[derive(Clone)]
pub struct AppState {
    pub db: &'static DatabaseConnection,
}

impl AppState {
    pub fn new(db: &'static DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(app_name: Option<&str>, router: Router<AppState>) -> anyhow::Result<()> {
    AppConfig::load(app_name.unwrap_or("app")).await?;
    logger::init().await;
    log::info!("Starting app server...");
    database::init().await?;
    let state = AppState::new(database::get().await);
    let server = server::Server::new(AppConfig::get().await.server());
    server.start(state, router).await
}
