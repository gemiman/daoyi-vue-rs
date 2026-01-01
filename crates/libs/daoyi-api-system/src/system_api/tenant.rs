use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::models::system::{IdParams, TenantPageReqVo};
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{TenantRespVO, TenantSaveReqVo, TenantUpdateReqVo};
use daoyi_entity_system::system_entity::system_tenant;
use daoyi_entity_system::system_service::system_tenant_service;
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/check-tenant-id", routing::post(check_tenant_id))
        .route("/get-by-website", routing::get(get_tenant_by_website))
        .route("/get-id-by-name", routing::get(get_tenant_id_by_name))
        .route("/simple-list", routing::get(get_tenant_simple_list))
        .route("/page", routing::get(get_tenant_page))
        .route("/create", routing::post(create_tenant))
        .route("/update", routing::put(update_tenant))
        .route("/get", routing::get(get_tenant))
}

#[debug_handler]
async fn update_tenant(ValidJson(vo): ValidJson<TenantUpdateReqVo>) -> RestApiResult<bool> {
    system_tenant_service::update_tenant(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_tenant(ValidJson(vo): ValidJson<TenantSaveReqVo>) -> RestApiResult<String> {
    ApiResponse::success(system_tenant_service::create_tenant(vo).await?.id)
}
#[debug_handler]
async fn get_tenant_page(
    ValidQuery(params): ValidQuery<TenantPageReqVo>,
) -> RestApiResult<Page<system_tenant::Model>> {
    ApiResponse::success(system_tenant_service::get_tenant_page(&params).await?)
}

#[debug_handler]
async fn get_tenant_simple_list() -> RestApiResult<Vec<TenantRespVO>> {
    let list = system_tenant_service::get_tenant_list_by_status(Some(CommonStatusEnum::Enable))
        .await?
        .into_iter()
        .map(|model| model.into())
        .collect();
    ApiResponse::success(list)
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTenantIdByNameParams {
    name: String,
}
#[debug_handler]
async fn get_tenant_id_by_name(
    ValidQuery(GetTenantIdByNameParams { name }): ValidQuery<GetTenantIdByNameParams>,
) -> RestApiResult<Option<String>> {
    if let Ok(model) = system_tenant_service::get_tenant_by_name(&name).await {
        return ApiResponse::success(Some(model.id));
    }
    ApiResponse::success(None)
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTenantByWebsiteParams {
    website: String,
}
#[debug_handler]
async fn get_tenant_by_website(
    ValidQuery(GetTenantByWebsiteParams { website }): ValidQuery<GetTenantByWebsiteParams>,
) -> RestApiResult<Option<TenantRespVO>> {
    if let Ok(model) = system_tenant_service::get_tenant_by_website(&website).await {
        if model.status == CommonStatusEnum::Disable {
            return ApiResponse::success(None);
        }
        return ApiResponse::success(Some(model.into()));
    }
    ApiResponse::success(None)
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CheckTenantParams {
    tenant_id: String,
}
#[debug_handler]
async fn check_tenant_id(
    ValidQuery(CheckTenantParams { tenant_id }): ValidQuery<CheckTenantParams>,
) -> RestApiResult<TenantRespVO> {
    ApiResponse::success(system_tenant_service::check_tenant_id(&tenant_id).await?)
}
#[debug_handler]
async fn get_tenant(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<TenantRespVO> {
    ApiResponse::success(system_tenant_service::get_tenant_by_id(&id).await?.into())
}
