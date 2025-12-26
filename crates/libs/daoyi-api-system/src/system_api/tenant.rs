use axum::{debug_handler, routing, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::TenantRespVO;
use daoyi_entity_system::system_service::system_tenant_service;
use serde::Deserialize;
use validator::Validate;
use daoyi_common_support::enumeration::CommonStatusEnum;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/check-tenant-id", routing::post(check_tenant_id))
        .route("/get-by-website", routing::get(get_tenant_by_website))
        .route("/get-id-by-name", routing::get(get_tenant_id_by_name))
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
