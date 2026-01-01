mod infra_api;
use axum::Router;
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin-api/infra", infra_api::create_router())
        .nest("/app-api/infra", infra_api::create_router())
}
