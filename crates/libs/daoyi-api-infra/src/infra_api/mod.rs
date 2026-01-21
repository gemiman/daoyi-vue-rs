mod file;
pub mod websocket;
pub mod ws;
mod codegen;

use axum::Router;
use axum::routing::get;
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/file-config", file::file_config::create_router())
        .nest("/file", file::file::create_router())
        .nest("/data-source-config", codegen::data_source_config::create_router())
        .nest("/codegen", codegen::codegen::create_router())
        .route("/ws", get(ws::ws_handler))
}