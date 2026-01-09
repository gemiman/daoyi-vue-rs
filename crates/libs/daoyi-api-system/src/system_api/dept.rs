use axum::extract::Query;
use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::request::valid::ValidJson;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    DeptListReqVO, DeptRespVo, DeptSaveReqVO, DeptSimpleRespVO, DeptUpdateReqVO,
};
use daoyi_entity_system::system_service::system_dept_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list", axum::routing::get(get_dept_list))
        .route("/list-all-simple", axum::routing::get(get_simple_dept_list))
        .route("/simple-list", axum::routing::get(get_simple_dept_list))
        .route("/create", axum::routing::post(create_dept))
        .route("/update", axum::routing::put(update_dept))
}

#[debug_handler]
async fn update_dept(ValidJson(vo): ValidJson<DeptUpdateReqVO>) -> RestApiResult<bool> {
    system_dept_service::update_dept(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_dept(ValidJson(vo): ValidJson<DeptSaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(system_dept_service::create_dept(vo).await?.id)
}

#[debug_handler]
async fn get_simple_dept_list() -> RestApiResult<Vec<DeptSimpleRespVO>> {
    let list = system_dept_service::get_dept_list_by_req(&DeptListReqVO {
        name: None,
        status: Some(CommonStatusEnum::Enable),
    })
    .await?;
    let vo_list = list.into_iter().map(|m| m.into()).collect();
    ApiResponse::success(vo_list)
}

#[debug_handler]
async fn get_dept_list(Query(req): Query<DeptListReqVO>) -> RestApiResult<Vec<DeptRespVo>> {
    let list = system_dept_service::get_dept_list_by_req(&req).await?;
    let vo_list = list.into_iter().map(|m| m.into()).collect();
    ApiResponse::success(vo_list)
}
