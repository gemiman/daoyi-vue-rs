use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::error::ApiError;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    PermissionAssignRoleDataScopeReqVO, PermissionAssignRoleMenuReqVO, RoleIdParams,
};
use daoyi_entity_system::system_service::{
    system_role_menu_service, system_role_service, system_tenant_service,
};
use std::sync::{Arc, Mutex};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list-role-menus", routing::get(get_role_menu_list))
        .route("/assign-role-menu", routing::post(assign_role_menu))
        .route(
            "/assign-role-data-scope",
            routing::post(assign_role_data_scope),
        )
}

#[debug_handler]
async fn assign_role_data_scope(
    ValidJson(vo): ValidJson<PermissionAssignRoleDataScopeReqVO>,
) -> RestApiResult<bool> {
    system_role_service::update_role_data_scope(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn assign_role_menu(
    ValidJson(vo): ValidJson<PermissionAssignRoleMenuReqVO>,
) -> RestApiResult<bool> {
    let menu_ids = vo.menu_ids.unwrap_or_default();
    // 开启多租户的情况下，需要过滤掉未开通的菜单
    let menus_arc = Arc::new(Mutex::new(menu_ids));
    let menus_clone = Arc::clone(&menus_arc);
    system_tenant_service::handle_tenant_menu_async(move |menu_ids| async move {
        let mut menus = menus_clone.lock().unwrap();
        menus.retain(|m| menu_ids.contains(&m));
        Ok(())
    })
    .await?;
    let menu_ids = Arc::try_unwrap(menus_arc)
        .map_err(|_| ApiError::biz("解包 Arc 失败"))?
        .into_inner()
        .map_err(|_| ApiError::biz("获取互斥锁内部数据失败"))?;
    // 执行菜单的分配
    system_role_menu_service::assign_role_menu(&vo.role_id, &menu_ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_role_menu_list(
    ValidQuery(RoleIdParams { role_id }): ValidQuery<RoleIdParams>,
) -> RestApiResult<Vec<String>> {
    ApiResponse::success(system_role_menu_service::get_role_menu_list_by_role_id(&role_id).await?)
}
