use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::vo::system_vo::TenantPackageRespVo;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_entity_system::system_service::system_tenant_package_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/simple-list", routing::get(get_tenant_package_list))
        .route("/get-simple-list", routing::get(get_tenant_package_list))
}

/// 获取租户套餐精简信息列表,只包含被开启的租户套餐，主要用于前端的下拉选项
#[debug_handler]
async fn get_tenant_package_list() -> RestApiResult<Vec<TenantPackageRespVo>> {
    let list =
        system_tenant_package_service::get_tenant_package_list_by_status(CommonStatusEnum::Enable)
            .await?
            .into_iter()
            .map(|item| item.into())
            .collect();
    ApiResponse::success(list)
}
