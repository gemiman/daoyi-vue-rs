use axum::Router;
use daoyi_common_support::app::AppState;

pub mod demo_api;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin-api/demo", demo_api::create_router())
        .nest("/app-api/demo", demo_api::create_router())
}
