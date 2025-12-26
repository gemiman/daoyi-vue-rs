use crate::system_entity::prelude::*;
use crate::system_entity::system_tenant;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::TenantRespVO;
use daoyi_common_support::{database, redis_utils};
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;

pub async fn get_tenant_by_id(tenant_id: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get().await;
    let option = SystemTenant::find_perm()
        .await
        .filter(system_tenant::Column::Id.eq(tenant_id))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(option)
}

pub async fn get_tenant_by_name(name: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get().await;
    let option = SystemTenant::find_perm()
        .await
        .filter(system_tenant::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(option)
}

pub async fn get_tenant_by_website(website: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get().await;
    let option = SystemTenant::find_perm()
        .await
        .filter(system_tenant::Column::Websites.eq(website))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(option)
}

pub async fn check_tenant_id(tenant_id: &str) -> ApiResult<TenantRespVO> {
    let redis_key = RedisKey::CheckTenantId.key(tenant_id);
    // 1. Try to get from Redis
    if let Some(vo) = redis_utils::cache_get_json::<TenantRespVO>(&redis_key).await? {
        return Ok(vo);
    }
    let model = get_tenant_by_id(tenant_id).await?;
    if model.status == CommonStatusEnum::Disable {
        return Err(ApiError::unauthenticated("租户被禁用"));
    }
    let vo: TenantRespVO = model.into();
    let now = Local::now().naive_local();
    let duration = vo.expire_time - now;
    let ttl = duration.num_seconds();
    if ttl > 0 {
        redis_utils::cache_set_json_ex(&redis_key, &vo, ttl as u64).await?;
    } else {
        return Err(ApiError::unauthenticated("租户过期"));
    }
    Ok(vo)
}
