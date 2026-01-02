use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::infra_vo::{FileConfigPageReqVO, FileConfigSaveReqVo};
use daoyi_entity_infra::infra_entity::infra_file_config;
use daoyi_entity_infra::infra_service::infra_file_config_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", routing::get(get_file_config_page))
        .route("/create", routing::post(create_file_config))
}

#[debug_handler]
async fn create_file_config(
    ValidJson(vo): ValidJson<FileConfigSaveReqVo>,
) -> RestApiResult<String> {
    ApiResponse::success(infra_file_config_service::create_file_config(vo).await?.id)
}

#[debug_handler]
async fn get_file_config_page(
    ValidQuery(params): ValidQuery<FileConfigPageReqVO>,
) -> RestApiResult<Page<infra_file_config::Model>> {
    ApiResponse::success(infra_file_config_service::get_file_config_page(&params).await?)
}
