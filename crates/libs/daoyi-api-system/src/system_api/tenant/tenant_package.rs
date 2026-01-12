use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, TenantPackagePageReqVO, TenantPackageRespVo, TenantPackageSaveReqVo,
    TenantPackageUpdateReqVo,
};
use daoyi_entity_system::system_entity::system_tenant_package;
use daoyi_entity_system::system_service::system_tenant_package_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/simple-list", routing::get(get_tenant_package_list))
        .route("/get-simple-list", routing::get(get_tenant_package_list))
        .route("/page", routing::get(get_tenant_package_page))
        .route("/get", routing::get(get_tenant_package))
        .route("/create", routing::post(create_tenant_package))
        .route("/update", routing::put(update_tenant_package))
}

#[debug_handler]
async fn update_tenant_package(
    ValidJson(vo): ValidJson<TenantPackageUpdateReqVo>,
) -> RestApiResult<bool> {
    system_tenant_package_service::update_tenant_package(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_tenant_package(
    ValidJson(vo): ValidJson<TenantPackageSaveReqVo>,
) -> RestApiResult<String> {
    let model = system_tenant_package_service::create_tenant_package(vo).await?;
    ApiResponse::success(model.id)
}

#[debug_handler]
async fn get_tenant_package(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<system_tenant_package::Model>> {
    ApiResponse::success(system_tenant_package_service::get_tenant_package(&id).await?)
}

#[debug_handler]
async fn get_tenant_package_page(
    ValidQuery(params): ValidQuery<TenantPackagePageReqVO>,
) -> RestApiResult<Page<system_tenant_package::Model>> {
    ApiResponse::success(system_tenant_package_service::get_tenant_package_page(&params).await?)
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
