use axum::Router;
use daoyi_common_support::app::AppState;

pub mod demo_api;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/demo", demo_api::create_router())
}
