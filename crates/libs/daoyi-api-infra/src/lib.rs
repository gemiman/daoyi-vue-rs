mod infra_api;
use crate::infra_api::ws;
use axum::Router;
use axum::routing::get;
use daoyi_common_support::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin-api/infra", infra_api::create_router())
        .nest("/app-api/infra", infra_api::create_router())
        .route("/infra/ws", get(ws::ws_handler))
}
