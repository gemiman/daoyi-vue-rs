use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::infra_vo::{
    FileConfigPageReqVO, FileConfigSaveReqVo, FileConfigUpdateReqVo,
};
use daoyi_common_support::vo::system_vo::{IdParams, IdsParams};
use daoyi_entity_infra::infra_entity::infra_file_config;
use daoyi_entity_infra::infra_service::infra_file_config_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", routing::get(get_file_config_page))
        .route("/create", routing::post(create_file_config))
        .route("/get", routing::get(get_file_config))
        .route("/update", routing::put(update_file_config))
        .route("/update-master", routing::put(update_file_config_master))
        .route("/delete", routing::delete(delete_file_config))
        .route("/delete-list", routing::delete(delete_file_config_list))
        .route("/test", routing::get(test_file_config))
}

#[debug_handler]
async fn test_file_config(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<String> {
    ApiResponse::success(infra_file_config_service::test_file_config(&id).await?)
}

#[debug_handler]
async fn delete_file_config_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    infra_file_config_service::delete_file_config_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_file_config(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    infra_file_config_service::delete_file_config(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_file_config_master(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    infra_file_config_service::update_file_config_master(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_file_config(
    ValidJson(vo): ValidJson<FileConfigUpdateReqVo>,
) -> RestApiResult<bool> {
    infra_file_config_service::update_file_config(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_file_config(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<infra_file_config::Model> {
    ApiResponse::success(infra_file_config_service::get_file_config(&id).await?)
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
) -> RestApiResult<PageResult<infra_file_config::Model>> {
    ApiResponse::success(infra_file_config_service::get_file_config_page(&params).await?)
}
