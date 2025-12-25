use axum::extract::ConnectInfo;
use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::error::ApiError;
use daoyi_common_support::password::verify_password;
use daoyi_common_support::request::valid::ValidJson;
use daoyi_common_support::response::{ApiResponse, ApiResult};
use daoyi_common_support::vo::system_vo::{AuthLoginReqVO, AuthLoginRespVO};
use daoyi_entity_system::system_service::{system_access_token_service, system_users_service};
use std::net::SocketAddr;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/login", routing::post(login))
        .route("/logout", routing::post(logout))
}

#[debug_handler]
#[tracing::instrument(name="login",skip_all,fields(ip=%addr.ip(),account=%params.username,password=%params.password))]
async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<AuthLoginReqVO>,
) -> ApiResult<AuthLoginRespVO> {
    tracing::info!("开始处理登录逻辑。。。");
    let user = system_users_service::get_by_username(&params.username)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码不正确")))?;
    if !verify_password(&params.password, &user.password).await? {
        return Err(ApiError::Biz(String::from("账号或密码不正确")));
    }
    tracing::info!(
        "登录成功，HttpRequestContext={:?}",
        HttpRequestContext::get_current()
    );
    let vo =
        system_access_token_service::create_token_after_login_success(&user.tenant_id, &user.id)
            .await?;
    ApiResponse::success(vo)
}

#[debug_handler]
async fn logout() -> ApiResult<()> {
    ApiResponse::success(())
}
