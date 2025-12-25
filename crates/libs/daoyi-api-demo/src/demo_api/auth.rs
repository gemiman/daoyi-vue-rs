use axum::extract::ConnectInfo;
use axum::{debug_handler, routing, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiError;
use daoyi_common_support::password::verify_password;
use daoyi_common_support::request::valid::ValidJson;
use daoyi_common_support::response::{ApiResponse, ApiResult};
use daoyi_entity_demo::demo_entity::prelude::*;
use daoyi_entity_demo::demo_entity::sys_user;
use sa_token_plugin_axum::*;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route("/login", routing::post(login))
        .route("/logout", routing::post(logout))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(min = 3, max = 16, message = "账号长度为3-16"))]
    account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度为6-16"))]
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    access_token: TokenValue,
}

#[debug_handler]
#[tracing::instrument(name="login",skip_all,fields(ip=%addr.ip(),account=%params.account,password=%params.password))]
async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> ApiResult<LoginResult> {
    tracing::info!("开始处理登录逻辑。。。");
    let db = database::get().await;
    let user = SysUser::find()
        .filter(sys_user::Column::Account.eq(params.account))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码不正确")))?;
    if !verify_password(&params.password, &user.password).await? {
        return Err(ApiError::Biz(String::from("账号或密码不正确")));
    }
    let access_token = StpUtil::login(&user.id).await?;
    tracing::info!("登录成功，access_token={access_token}");
    // 设置权限和角色
    StpUtil::set_permissions(
        &user.id,
        vec!["user:list".to_string(), "user:add".to_string()],
    )
    .await?;

    StpUtil::set_roles(&user.id, vec!["admin".to_string()]).await?;
    ApiResponse::success(LoginResult { access_token })
}

#[debug_handler]
async fn logout() -> ApiResult<()> {
    let x = StpUtil::logout_current().await?;
    ApiResponse::success(x)
}

#[debug_handler]
#[sa_check_login]
async fn get_user_info(LoginIdExtractor(login_id): LoginIdExtractor) -> ApiResult<sys_user::Model> {
    let user = SysUser::find_by_id(login_id)
        .one(database::get().await)
        .await?;
    Ok(ApiResponse::ok(user))
}
