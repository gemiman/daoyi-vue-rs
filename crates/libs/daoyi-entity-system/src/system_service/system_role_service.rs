use crate::system_entity::prelude::*;
use crate::system_entity::system_role;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::enumeration::RoleCodeEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::{database, redis_utils};
use sea_orm::prelude::*;

pub async fn get_role_list_by_ids(ids: &Vec<String>) -> ApiResult<Vec<system_role::Model>> {
    let db = database::get().await;
    let list = SystemRole::find_perm()
        .await
        .filter(system_role::Column::Id.is_in(ids))
        .all(db)
        .await?;
    Ok(list)
}

pub async fn has_any_super_admin(ids: &Vec<String>) -> ApiResult<bool> {
    if ids.is_empty() {
        return Ok(false);
    }
    for id in ids {
        if let Ok(role) = get_role_from_cache(id).await {
            if RoleCodeEnum::is_super_admin(&role.code) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub async fn get_role_by_id(id: &str) -> ApiResult<system_role::Model> {
    let db = database::get().await;
    let role = SystemRole::find_perm()
        .await
        .filter(system_role::Column::Id.eq(id))
        .one(db)
        .await?
        .ok_or(ApiError::biz("角色不存在"))?;
    Ok(role)
}

pub async fn get_role_from_cache(id: &str) -> ApiResult<system_role::Model> {
    let redis_key = RedisKey::RoleById.key(id);
    // 1. Try to get from Redis
    if let Some(role) = redis_utils::cache_get_json::<system_role::Model>(&redis_key).await? {
        return Ok(role);
    }
    let role = get_role_by_id(id).await?;
    redis_utils::cache_set_json(&redis_key, &role).await?;
    Ok(role)
}
