use axum::{Router, routing};
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
}
