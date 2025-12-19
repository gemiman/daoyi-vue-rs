use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", routing::get(index));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "Server is listening on: http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}
