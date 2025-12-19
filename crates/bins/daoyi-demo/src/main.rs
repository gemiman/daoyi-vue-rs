use axum::{Router, debug_handler, routing};
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::logger::{self, log};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init().await;
    let port = AppConfig::get().await.server().port();
    let router = Router::new().route("/", routing::get(index));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    log::info!(
        "Server is listening on: http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}
