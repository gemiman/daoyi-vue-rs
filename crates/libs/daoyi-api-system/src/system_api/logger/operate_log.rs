use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::IdParams;
use daoyi_common_support::vo::system_vo::operate_log_vo::{OperateLogPageReqVO, OperateLogRespVO};
use daoyi_entity_system::system_service::system_operate_log_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/get", routing::get(get_operate_log))
        .route("/page", routing::get(get_operate_log_page))
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
