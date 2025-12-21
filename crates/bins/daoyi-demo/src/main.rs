use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, debug_handler, routing};
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::database;
use daoyi_common_support::logger::{self, log};
use daoyi_entity_demo::demo_entity::prelude::*;
use sea_orm::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AppConfig::load(env!("CARGO_PKG_NAME")).await?;
    logger::init().await;
    let db = database::init().await?;
    let port = AppConfig::get().await.server().port();
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(query_users))
        .with_state(db);
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    log::info!("Server is listening on: http://127.0.0.1:{}", port);
    axum::serve(listener, router).await?;
    Ok(())
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}

#[debug_handler]
async fn query_users(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let users = SysUser::find().all(&db).await.unwrap();
    axum::Json(users)
}
