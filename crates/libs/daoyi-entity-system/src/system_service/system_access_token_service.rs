use crate::system_entity::prelude::*;
use crate::system_entity::system_access_token;
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::AuthLoginRespVO;
use daoyi_common_support::{database, redis_utils};
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::Set;

pub async fn get_access_token(token: &str) -> ApiResult<system_access_token::Model> {
    let db = database::get().await;
    let option = SystemAccessToken::find_perm()
        .await
        .filter(system_access_token::Column::AccessToken.eq(token))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::biz("Token不存在"))?;
    Ok(option)
}

pub async fn check_access_token(token: &str) -> ApiResult<AuthLoginRespVO> {
    let redis_key = RedisKey::CheckToken.key(token);
    // 1. Try to get from Redis
    if let Some(vo) = redis_utils::cache_get_json::<AuthLoginRespVO>(&redis_key).await? {
        return Ok(vo);
    }
    let vo: AuthLoginRespVO = get_access_token(token).await?.into();
    let now = Local::now().naive_local();
    let duration = vo.expires_time - now;
    let ttl = duration.num_seconds();
    if ttl > 0 {
        redis_utils::cache_set_json_ex(&redis_key, &vo, ttl as u64).await?;
    } else {
        return Err(ApiError::unauthenticated("Token过期"));
    }
    Ok(vo)
}

pub async fn create_token_after_login_success(
    tenant_id: &str,
    login_id: &str,
) -> ApiResult<AuthLoginRespVO> {
    let access_token = loop {
        let token = xid::new().to_string();
        if let Err(_) = get_access_token(&token).await {
            break token;
        }
    };
    let mut context = HttpRequestContext::new();
    context.token = Some(access_token.clone());
    context.login_id = Some(String::from(login_id));
    context.tenant_id = Some(String::from(tenant_id));
    HttpRequestContext::set_current(context);
    let token_expiration = AppConfig::get().await.auth().token_expiration();
    let db = database::get().await;
    let mut active_model = system_access_token::ActiveModel::new();
    active_model.user_id = Set(String::from(login_id));
    active_model.access_token = Set(access_token);
    active_model.expires_time = Set(Local::now().naive_local() + token_expiration);
    let model = active_model.insert(db).await?;
    Ok(model.into())
}
