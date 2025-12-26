use axum::{debug_handler, routing, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::TenantRespVO;
use daoyi_entity_system::system_service::system_tenant_service;
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/check-tenant-id", routing::post(check_tenant_id))
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
