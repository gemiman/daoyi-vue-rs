use axum::extract::ConnectInfo;
use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiError;
use daoyi_common_support::password::verify_password;
use daoyi_common_support::request::valid::ValidJson;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_entity_demo::demo_entity::prelude::*;
use daoyi_entity_demo::demo_entity::sys_user;
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
    access_token: String,
}

#[debug_handler]
#[tracing::instrument(name="login",skip_all,fields(ip=%addr.ip(),account=%params.account,password=%params.password))]
async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> RestApiResult<LoginResult> {
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
    let access_token = xid::new().to_string();
    tracing::info!("登录成功，access_token={access_token}");
    ApiResponse::success(LoginResult { access_token })
}

#[debug_handler]
async fn logout() -> RestApiResult<()> {
    ApiResponse::success(())
}

#[debug_handler]
async fn get_user_info() -> RestApiResult<sys_user::Model> {
    let user = SysUser::find_by_id("1").one(database::get().await).await?;
    Ok(ApiResponse::ok(user))
}
