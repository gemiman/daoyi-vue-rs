use axum::Router;
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(daoyi_api_system::create_router())
}
