use axum::{Router, debug_handler, routing};
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::logger::{self, log};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AppConfig::load(env!("CARGO_PKG_NAME")).await?;
    logger::init().await;
    let port = AppConfig::get().await.server().port();
    let router = Router::new().route("/", routing::get(index));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    log::info!("Server is listening on: http://127.0.0.1:{}", port);

    axum::serve(listener, router).await?;
    Ok(())
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}
