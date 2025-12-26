use axum::{debug_handler, routing, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::AuthLoginRespVO;
use daoyi_entity_system::system_service::system_access_token_service;
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/check-token", routing::post(check_token))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CheckTokenParams {
    token: String,
}
#[debug_handler]
async fn check_token(
    ValidQuery(CheckTokenParams { token }): ValidQuery<CheckTokenParams>,
) -> RestApiResult<AuthLoginRespVO> {
    ApiResponse::success(system_access_token_service::check_access_token(&token).await?)
}
