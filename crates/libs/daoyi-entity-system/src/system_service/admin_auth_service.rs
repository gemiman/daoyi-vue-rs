use crate::system_entity::system_access_token;
use crate::system_service::system_access_token_service::get_access_token;
use crate::system_service::{
    system_access_token_service, system_login_log_service, system_users_service,
};
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::enumeration::{LoginLogTypeEnum, LoginResultEnum, UserTypeEnum};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::password::verify_password;
use daoyi_common_support::vo::system_vo::login_log_vo::LoginLogCreateReqDTO;
use daoyi_common_support::vo::system_vo::{AuthLoginReqVO, AuthLoginRespVO};
use daoyi_common_support::{database, id_util};
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, Set};
use std::sync::Arc;

pub async fn login(vo: AuthLoginReqVO) -> ApiResult<AuthLoginRespVO> {
    // 使用账号密码，进行登录
    let user = system_users_service::get_by_username(&vo.username)
        .await?
        .ok_or_else(|| ApiError::biz("账号或密码不正确"))?;
    if !verify_password(&vo.password, &user.password).await? {
        return Err(ApiError::biz("账号或密码不正确"));
    }
    // 创建 Token 令牌，记录登录日志
    let vo = create_token_after_login_success(
        &user.tenant_id,
        &user.id,
        &user.username,
        LoginLogTypeEnum::LoginUsername,
    )
    .await?;
    Ok(vo)
}

async fn create_login_log(
    user_id: &str,
    username: &str,
    log_type: LoginLogTypeEnum,
    login_result: LoginResultEnum,
) -> ApiResult<()> {
    let vo = LoginLogCreateReqDTO {
        log_type,
        trace_id: HttpRequestContext::get_tracing_id_as_string(),
        user_id: user_id.to_string(),
        user_type: UserTypeEnum::Admin,
        username: username.to_string(),
        result: login_result,
        user_ip: HttpRequestContext::get_user_ip_as_string(),
        user_agent: HttpRequestContext::get_user_agent_as_string(),
    };
    system_login_log_service::create_login_log(vo).await?;
    Ok(())
}

async fn create_token_after_login_success(
    tenant_id: &str,
    login_id: &str,
    username: &str,
    log_type: LoginLogTypeEnum,
) -> ApiResult<AuthLoginRespVO> {
    // 插入登陆日志
    create_login_log(login_id, username, log_type, LoginResultEnum::Success).await?;
    // 创建访问令牌
    let access_token = loop {
        let token = id_util::xid();
        if let Err(_) = get_access_token(&token).await {
            break token;
        }
    };
    let refresh_token = loop {
        let token = id_util::xid();
        if let Err(_) = system_access_token_service::get_refresh_token(&token).await {
            break token;
        }
    };
    let mut context = HttpRequestContext::new();
    context.token = Some(Arc::new(access_token.clone()));
    context.login_id = Some(Arc::new(String::from(login_id)));
    context.tenant_id = Some(Arc::new(String::from(tenant_id)));

    HttpRequestContext::scope(context, || async move {
        let token_expiration = AppConfig::get().auth().token_expiration();
        let db = database::get_db_async().await;
        let mut active_model = system_access_token::ActiveModel::new();
        active_model.user_id = Set(String::from(login_id));
        active_model.access_token = Set(access_token);
        active_model.refresh_token = Set(refresh_token);
        active_model.expires_time = Set(Local::now().naive_local() + token_expiration);
        let model = active_model.insert(&db).await?;
        Ok(model.into())
    })
    .await
}
