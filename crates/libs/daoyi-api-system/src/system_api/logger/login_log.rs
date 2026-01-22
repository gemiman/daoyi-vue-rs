use axum::{debug_handler, Router};
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
}


#[debug_handler]
async fn get_login_log_page() {}