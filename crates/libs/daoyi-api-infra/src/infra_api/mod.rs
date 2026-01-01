mod file;

use axum::Router;
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/file-config", file::file_config::create_router())
        .nest("/file", file::file::create_router())
}
