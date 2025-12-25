mod system_api;

use axum::Router;
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin-api/system", system_api::create_router())
        .nest("/app-api/system", system_api::create_router())
}
