use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::IdParams;
use daoyi_common_support::vo::system_vo::operate_log_vo::{
    OperateLogCreateReqDTO, OperateLogPageReqVO, OperateLogRespVO,
};
use daoyi_entity_system::system_service::system_operate_log_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/get", routing::get(get_operate_log))
        .route("/page", routing::get(get_operate_log_page))
        .route("/create", routing::post(create_operate_log))
}

#[debug_handler]
async fn create_operate_log(
    ValidJson(vo): ValidJson<OperateLogCreateReqDTO>,
) -> RestApiResult<String> {
    ApiResponse::success(system_operate_log_service::create_operate_log(vo).await?.id)
}

#[debug_handler]
async fn get_operate_log_page(
    ValidQuery(params): ValidQuery<OperateLogPageReqVO>,
) -> RestApiResult<PageResult<OperateLogRespVO>> {
    ApiResponse::success(system_operate_log_service::get_operate_log_page(&params).await?)
}

#[debug_handler]
async fn get_operate_log(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<OperateLogRespVO>> {
    ApiResponse::success(
        system_operate_log_service::get_operate_log(&id)
            .await?
            .map(Into::into),
    )
}
