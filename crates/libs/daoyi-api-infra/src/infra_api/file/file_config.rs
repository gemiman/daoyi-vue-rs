use axum::{debug_handler, routing, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::infra_vo::FileConfigPageReqVO;
use daoyi_entity_infra::infra_entity::infra_file_config;
use daoyi_entity_infra::infra_service::infra_file_config_service;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/page", routing::get(get_file_config_page))
}

#[debug_handler]
async fn get_file_config_page(
    ValidQuery(params): ValidQuery<FileConfigPageReqVO>,
) -> RestApiResult<Page<infra_file_config::Model>> {
    ApiResponse::success(infra_file_config_service::get_file_config_page(&params).await?)
}
