use axum::{debug_handler, routing, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_entity_system::system_entity::system_menu;
use daoyi_entity_system::system_service::system_menu_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/simple-list", routing::get(get_simple_menu_list))
        .route("/get-simple-list", routing::get(get_simple_menu_list))
}

#[debug_handler]
async fn get_simple_menu_list() -> RestApiResult<Vec<system_menu::Model>> {
    ApiResponse::success(system_menu_service::get_simple_menu_list().await?)
}
