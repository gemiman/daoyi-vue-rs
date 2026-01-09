use axum::extract::Query;
use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{DeptListReqVO, DeptRespVo};
use daoyi_entity_system::system_service::system_dept_service;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/list", axum::routing::get(get_menu_list))
}

#[debug_handler]
async fn get_menu_list(Query(req): Query<DeptListReqVO>) -> RestApiResult<Vec<DeptRespVo>> {
    let list = system_dept_service::get_dept_list_by_req(&req).await?;
    let vo_list = list.into_iter().map(|m| m.into()).collect();
    ApiResponse::success(vo_list)
}
