use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::request::valid::ValidJson;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    AuthLoginReqVO, AuthLoginRespVO, AuthPermissionInfoRespVO,
};
use daoyi_entity_system::system_entity::system_role;
use daoyi_entity_system::system_service::{
    admin_auth_service, system_menu_service, system_role_menu_service, system_role_service,
    system_user_role_service, system_users_service,
};
use std::collections::HashSet;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/login", routing::post(login))
        .route("/logout", routing::post(logout))
        .route("/get-permission-info", routing::get(get_permission_info))
}

#[debug_handler]
async fn get_permission_info() -> RestApiResult<AuthPermissionInfoRespVO> {
    let mut vo = AuthPermissionInfoRespVO::default();
    let login_user_id = HttpRequestContext::get_login_id_arc();
    if login_user_id.is_none() {
        return ApiResponse::success(vo);
    }
    let login_user_id = login_user_id.unwrap();
    // 1.1 获得用户信息
    let user = system_users_service::get_by_id(login_user_id.as_str()).await;
    if user.is_err() {
        return ApiResponse::success(vo);
    }
    vo.user = user?.into();
    // 1.2 获得角色列表
    let role_ids =
        system_user_role_service::get_user_role_id_list_by_user_id(login_user_id.as_str()).await?;
    if role_ids.is_empty() {
        return ApiResponse::success(vo);
    }
    let roles = system_role_service::get_role_list_by_ids(&role_ids)
        .await?
        .into_iter()
        .filter(|r| r.status == CommonStatusEnum::Enable)
        .collect::<Vec<system_role::Model>>();
    let role_codes = roles.iter().map(|r| r.code.to_owned()).collect();
    vo.roles = role_codes;
    let role_ids = roles.iter().map(|r| r.id.to_owned()).collect::<Vec<_>>();
    // 1.3 获得菜单列表
    let menu_ids = system_role_menu_service::get_role_menu_list_by_role_ids(&role_ids).await?;
    let menu_list = system_menu_service::get_menu_list(Some(&menu_ids))
        .await?
        .into_iter()
        .filter(|m| m.status == CommonStatusEnum::Enable)
        .collect::<Vec<_>>();
    vo.permissions = menu_list
        .iter()
        .filter_map(|m| m.permission.clone())
        .filter(|p| !p.is_empty())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    vo.menus = system_menu_service::build_menu_tree(menu_list).await?;
    // 2. 拼接结果返回
    ApiResponse::success(vo)
}

#[debug_handler]
async fn login(ValidJson(params): ValidJson<AuthLoginReqVO>) -> RestApiResult<AuthLoginRespVO> {
    ApiResponse::success(admin_auth_service::login(params).await?)
}

#[debug_handler]
async fn logout() -> RestApiResult<()> {
    ApiResponse::success(())
}
