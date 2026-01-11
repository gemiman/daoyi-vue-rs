use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, UserPageReqVO, UserRespVO, UserSaveReqVO, UserSimpleRespVo, UserUpdateReqVO,
};
use daoyi_entity_system::system_service::{system_dept_service, system_users_service};
use std::collections::HashSet;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list-all-simple", axum::routing::get(get_simple_user_list))
        .route("/simple-list", axum::routing::get(get_simple_user_list))
        .route("/page", axum::routing::get(get_user_page))
        .route("/export-excel", axum::routing::get(get_user_page))
        .route("/get", axum::routing::get(get_user))
        .route("/create", axum::routing::post(create_user))
        .route("/update", axum::routing::put(update_user))
}

#[debug_handler]
async fn update_user(ValidJson(vo): ValidJson<UserUpdateReqVO>) -> RestApiResult<bool> {
    system_users_service::update_user(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_user(ValidJson(vo): ValidJson<UserSaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(system_users_service::create_user(vo).await?.id)
}

#[debug_handler]
async fn get_user(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<UserRespVO> {
    let user = system_users_service::get_by_id(&id).await?;
    if let Some(dept_id) = user.dept_id.as_deref() {
        // 拼接数据
        let dept = system_dept_service::get_dept(dept_id).await?;
        return ApiResponse::success(user.convert_vo(dept.map(|d| d.name)));
    }
    ApiResponse::success(user.convert_vo(None))
}

#[debug_handler]
async fn get_user_page(
    ValidQuery(params): ValidQuery<UserPageReqVO>,
) -> RestApiResult<Page<UserRespVO>> {
    ApiResponse::success(system_users_service::get_user_page(&params).await?)
}

#[debug_handler]
async fn get_simple_user_list() -> RestApiResult<Vec<UserSimpleRespVo>> {
    let list = system_users_service::get_user_list_by_status(CommonStatusEnum::Enable).await?;
    if list.is_empty() {
        return ApiResponse::success(vec![]);
    }
    let dept_ids = list
        .iter()
        .filter(|x| x.dept_id.is_some())
        .map(|x| x.dept_id.as_deref().unwrap())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let dept_map = system_dept_service::get_dept_map(dept_ids).await?;
    let list = list
        .into_iter()
        .map(|u| {
            let dept_name = u
                .dept_id
                .as_ref()
                .and_then(|dept_id| dept_map.get(dept_id).map(|d| d.name.clone()));
            u.convert_simple_vo(dept_name)
        })
        .collect();
    ApiResponse::success(list)
}
