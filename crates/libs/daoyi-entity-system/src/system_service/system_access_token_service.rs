use crate::system_entity::prelude::*;
use crate::system_entity::system_access_token;
use daoyi_common_support::configs::AppConfig;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::AuthLoginRespVO;
use daoyi_common_support::{database, redis_utils};
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;

pub async fn get_refresh_token(token: &str) -> ApiResult<system_access_token::Model> {
    let db = database::get_db_async().await;
    let option = SystemAccessToken::find_perm()
        .await
        .filter(system_access_token::Column::RefreshToken.eq(token))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::biz("Token不存在"))?;
    Ok(option)
}

pub async fn get_access_token(token: &str) -> ApiResult<system_access_token::Model> {
    let db = database::get_db_async().await;
    let option = SystemAccessToken::find_perm()
        .await
        .filter(system_access_token::Column::AccessToken.eq(token))
        .one(&db)
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
    let vo: AuthLoginRespVO = if let Ok(model) = get_access_token(token).await {
        model
    } else {
        let mut model = get_refresh_token(token).await?;
        model.expires_time =
            Local::now().naive_local() + AppConfig::get().auth().token_expiration();
        model
    }
    .into();
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
