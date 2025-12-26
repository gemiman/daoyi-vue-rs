use crate::configs::AppConfig;
use crate::enumeration::redis_keys::RedisKey;
use crate::error::{ApiError, ApiResult};
use crate::redis_utils;
use crate::response::ApiResponse;
use crate::vo::system_vo::{AuthLoginRespVO, TenantRespVO};
use sea_orm::sqlx::types::chrono::Local;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Principal {
    pub id: String,
    pub name: String,
}

pub async fn check_token(token: &str) -> ApiResult<AuthLoginRespVO> {
    let redis_key = RedisKey::CheckToken.key(token);

    // 1. Try to get from Redis
    if let Some(vo) = redis_utils::cache_get_json::<AuthLoginRespVO>(&redis_key).await? {
        return Ok(vo);
    }

    // 2. Call remote check URL
    let token_check_url = AppConfig::get().await.auth().token_check_url();
    let client = reqwest::Client::new();

    let resp = client
        .post(token_check_url)
        .query(&[("token", token)])
        .send()
        .await
        .map_err(|e| ApiError::unauthenticated(format!("Token校验失败: {}", e)))?;

    if !resp.status().is_success() {
        return Err(ApiError::unauthenticated(format!(
            "Token校验失败：status: {}",
            resp.status()
        )));
    }

    let api_response = resp
        .json::<ApiResponse<AuthLoginRespVO>>()
        .await
        .map_err(|e| ApiError::unauthenticated(format!("Token校验失败: {}", e)))?;

    if !api_response.success {
        return Err(ApiError::unauthenticated(api_response.message));
    }

    let vo = api_response
        .data
        .ok_or_else(|| ApiError::unauthenticated("Token校验失败"))?;

    // 3. Cache the result
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

pub async fn check_tenant_id(tenant_id: &str) -> ApiResult<TenantRespVO> {
    let redis_key = RedisKey::CheckTenantId.key(tenant_id);

    // 1. Try to get from Redis
    if let Some(vo) = redis_utils::cache_get_json::<TenantRespVO>(&redis_key).await? {
        return Ok(vo);
    }

    // 2. Call remote check URL
    let tenant_check_url = AppConfig::get().await.auth().tenant_check_url();
    let client = reqwest::Client::new();

    let resp = client
        .post(tenant_check_url)
        .query(&[("tenantId", tenant_id)])
        .send()
        .await
        .map_err(|e| ApiError::unauthenticated(format!("租户校验失败: {}", e)))?;

    if !resp.status().is_success() {
        return Err(ApiError::unauthenticated(format!(
            "租户校验失败：status: {}",
            resp.status()
        )));
    }

    let api_response = resp
        .json::<ApiResponse<TenantRespVO>>()
        .await
        .map_err(|e| ApiError::unauthenticated(format!("租户校验失败: {}", e)))?;

    if !api_response.success {
        return Err(ApiError::unauthenticated(api_response.message));
    }

    let vo = api_response
        .data
        .ok_or_else(|| ApiError::unauthenticated("租户校验失败"))?;

    // 3. Cache the result
    let now = Local::now().naive_local();
    let duration = vo.expire_time - now;
    let ttl = duration.num_seconds();

    if ttl > 0 {
        redis_utils::cache_set_json_ex(&redis_key, &vo, ttl as u64).await?;
    } else {
        return Err(ApiError::unauthenticated("Token过期"));
    }

    Ok(vo)
}
