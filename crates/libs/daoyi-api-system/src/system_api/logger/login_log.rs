use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::login_log_vo::{LoginLogPageReqVO, LoginLogRespVO};
use daoyi_entity_system::system_service::system_login_log_service;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/page", routing::get(get_login_log_page))
}

#[debug_handler]
async fn get_login_log_page(
    ValidQuery(params): ValidQuery<LoginLogPageReqVO>,
) -> RestApiResult<PageResult<LoginLogRespVO>> {
    ApiResponse::success(system_login_log_service::get_login_log_page(&params).await?)
}
