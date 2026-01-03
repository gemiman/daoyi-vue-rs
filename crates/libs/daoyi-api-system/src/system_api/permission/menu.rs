use axum::extract::Query;
use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, MenuListReqVO, MenuRespVO, MenuSaveVO, MenuSimpleRespVo, MenuUpdateVO,
};
use daoyi_entity_system::system_entity::system_menu;
use daoyi_entity_system::system_service::system_menu_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/create", routing::post(create_menu))
        .route("/update", routing::put(update_menu))
        .route("/delete", routing::delete(delete_menu))
        .route("/delete-list", routing::delete(delete_menu_list))
        .route("/list", routing::get(get_menu_list))
        .route("/get", routing::get(get_menu))
        .route("/simple-list", routing::get(get_simple_menu_list))
        .route("/list-all-simple", routing::get(get_simple_menu_list))
}

#[debug_handler]
async fn create_menu(ValidJson(req): ValidJson<MenuSaveVO>) -> RestApiResult<String> {
    ApiResponse::success(system_menu_service::create_menu(req).await?)
}

#[debug_handler]
async fn update_menu(ValidJson(req): ValidJson<MenuUpdateVO>) -> RestApiResult<bool> {
    system_menu_service::update_menu(req).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_menu(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<bool> {
    system_menu_service::delete_menu(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_menu_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_menu_service::delete_menu_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_menu_list(Query(req): Query<MenuListReqVO>) -> RestApiResult<Vec<MenuRespVO>> {
    let list = system_menu_service::get_menu_list_by_req(&req).await?;
    let mut vo_list: Vec<MenuRespVO> = list.into_iter().map(|m| m.into()).collect();
    vo_list.sort_by_key(|m| m.sort);
    ApiResponse::success(vo_list)
}

#[debug_handler]
async fn get_menu(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<system_menu::Model>> {
    ApiResponse::success(system_menu_service::get_menu(&id).await?)
}

/// 获取菜单精简信息列表
/// 只包含被开启的菜单，用于【角色分配菜单】功能的选项。在多租户的场景下，会只返回租户所在套餐有的菜单
#[debug_handler]
async fn get_simple_menu_list() -> RestApiResult<Vec<MenuSimpleRespVo>> {
    let vec = system_menu_service::get_menu_list_by_tenant(Some(CommonStatusEnum::Enable)).await?;
    let vec: Vec<MenuSimpleRespVo> = vec.into_iter().map(|m| m.into()).collect();
    ApiResponse::success(vec)
}
