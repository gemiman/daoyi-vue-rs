use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{AuthLoginRespVO, TokenParams};
use daoyi_entity_system::system_service::system_access_token_service;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/check-token", routing::post(check_token))
}

#[debug_handler]
async fn check_token(
    ValidQuery(TokenParams { token }): ValidQuery<TokenParams>,
) -> RestApiResult<AuthLoginRespVO> {
    ApiResponse::success(system_access_token_service::check_access_token(&token).await?)
}
