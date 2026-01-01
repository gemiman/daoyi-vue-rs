use crate::system_entity::prelude::*;
use crate::system_entity::system_role;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::enumeration::{DataScopeEnum, RoleCodeEnum, RoleTypeEnum};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::vo::system_vo::RoleSaveReqVo;
use daoyi_common_support::{database, redis_utils};
use daoyi_macros::transactional;
use sea_orm::Set;
use sea_orm::prelude::*;

#[transactional]
pub async fn create_role(
    req_vo: RoleSaveReqVo,
    role_type: Option<RoleTypeEnum>,
) -> ApiResult<String> {
    // 1. 校验角色
    validate_role_duplicate(&req_vo.name, &req_vo.code, req_vo.id.as_deref()).await?;
    // 2. 插入到数据库
    let db = database::get_db_async().await;
    let mut active_model: system_role::ActiveModel = req_vo.into();
    active_model.r#type = Set(role_type.unwrap_or(RoleTypeEnum::CUSTOM));
    active_model.data_scope = Set(DataScopeEnum::ALL); // 默认可查看所有数据。原因是，可能一些项目不需要项目权限
    Ok(active_model.insert(&db).await?.id)
}

async fn validate_role_duplicate(name: &str, code: &str, id: Option<&str>) -> ApiResult<()> {
    // 0. 超级管理员，不允许创建
    if RoleCodeEnum::is_super_admin(code) {
        return Err(ApiError::biz(format!("标识【{}】不能使用", code)));
    }
    let db = database::get_db_async().await;
    // 1. 该 name 名字被其它角色所使用
    let role = SystemRole::find_perm()
        .await
        .filter(system_role::Column::Name.eq(name))
        .one(&db)
        .await?;
    if let Some(role) = role {
        if let Some(id) = id
            && role.id != id
        {
            return Err(ApiError::biz(format!("已经存在名为【{}】的角色", name)));
        }
        if id.is_none() {
            return Err(ApiError::biz(format!("已经存在名为【{}】的角色", name)));
        }
    }
    // 2. 是否存在相同编码的角色
    let role = SystemRole::find_perm()
        .await
        .filter(system_role::Column::Code.eq(code))
        .one(&db)
        .await?;
    if let Some(role) = role {
        if let Some(id) = id
            && role.id != id
        {
            return Err(ApiError::biz(format!("已经存在标识为【{}】的角色", code)));
        }
        if id.is_none() {
            return Err(ApiError::biz(format!("已经存在标识为【{}】的角色", code)));
        }
    }
    Ok(())
}
pub async fn get_role_list_by_ids(ids: &Vec<String>) -> ApiResult<Vec<system_role::Model>> {
    let db = database::get_db_async().await;
    let list = SystemRole::find_perm()
        .await
        .filter(system_role::Column::Id.is_in(ids))
        .all(&db)
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
    let db = database::get_db_async().await;
    let role = SystemRole::find_perm()
        .await
        .filter(system_role::Column::Id.eq(id))
        .one(&db)
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

/// 获得所有角色列表
pub async fn get_role_list() -> ApiResult<Vec<system_role::Model>> {
    let db = database::get_db_async().await;
    let list = SystemRole::find_perm().await.all(&db).await?;
    Ok(list)
}
