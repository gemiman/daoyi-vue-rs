use crate::system_entity::prelude::*;
use crate::system_entity::system_role;
use crate::system_service::{system_role_menu_service, system_user_role_service};
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::enumeration::{
    CommonStatusEnum, DataScopeEnum, RoleCodeEnum, RoleTypeEnum,
};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::vo::system_vo::{
    PermissionAssignRoleDataScopeReqVO, RolePageReqVO, RoleSaveReqVO, RoleUpdateReqVO,
};
use daoyi_common_support::{database, redis_utils};
use daoyi_macros::transactional;
use futures::future::try_join_all;
use sea_orm::prelude::*;
use sea_orm::{IntoActiveModel, QueryOrder, QueryTrait, Set};

#[transactional]
pub async fn create_role(
    req_vo: RoleSaveReqVO,
    role_type: Option<RoleTypeEnum>,
) -> ApiResult<system_role::Model> {
    // 1. 校验角色
    validate_role_duplicate(&req_vo.name, &req_vo.code, None).await?;
    // 2. 插入到数据库
    let db = database::get_db_async().await;
    let mut active_model: system_role::ActiveModel = req_vo.into();
    active_model.r#type = Set(role_type.unwrap_or(RoleTypeEnum::CUSTOM));
    active_model.data_scope = Set(DataScopeEnum::ALL); // 默认可查看所有数据。原因是，可能一些项目不需要项目权限
    Ok(active_model.insert(&db).await?)
}

pub async fn update_role(vo: RoleUpdateReqVO) -> ApiResult<()> {
    // 1.1 校验是否可以更新
    validate_role_for_update(&vo.id).await?;
    // 1.2 校验角色的唯一字段是否重复
    validate_role_duplicate(&vo.name, &vo.code, Some(&vo.id)).await?;
    // 2. 更新到数据库
    let db = database::get_db_async().await;
    let active_model: system_role::ActiveModel = vo.into();
    active_model.update(&db).await?;
    Ok(())
}

async fn validate_role_for_update(id: &str) -> ApiResult<system_role::Model> {
    let model = get_role_by_id(id).await?;
    // 内置角色，不允许删除
    if RoleTypeEnum::SYSTEM == model.r#type {
        return Err(ApiError::biz("不能操作类型为系统内置的角色"));
    }
    Ok(model)
}

async fn validate_role_duplicate(name: &str, code: &str, id: Option<&str>) -> ApiResult<()> {
    // 0. 超级管理员，不允许创建
    if RoleCodeEnum::is_super_admin(code) {
        return Err(ApiError::biz(format!("标识【{}】不能使用", code)));
    }
    let db = database::get_db_async().await;
    // 1. 该 name 名字被其它角色所使用
    let role = SystemRole::find_perm_with_tenant()
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
    let role = SystemRole::find_perm_with_tenant()
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
    let list = SystemRole::find_perm_with_tenant()
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
    let role = SystemRole::find_by_id_perm_with_tenant(&db, id)
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
    let list = SystemRole::find_perm_with_tenant().await.all(&db).await?;
    Ok(list)
}

pub async fn get_role_page(params: &RolePageReqVO) -> ApiResult<Page<system_role::Model>> {
    let db = database::get_db_async().await;
    let paginator = SystemRole::find_perm_with_tenant()
        .await
        .apply_if(params.name.as_ref(), |query, name| {
            query.filter(system_role::Column::Name.contains(name))
        })
        .apply_if(params.code.as_ref(), |query, code| {
            query.filter(system_role::Column::Code.contains(code))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_role::Column::Status.eq(status))
        })
        .apply_if(params.create_time.as_ref(), |query, create_time| {
            query.filter(system_role::Column::CreateTime.between(create_time[0], create_time[1]))
        })
        .order_by_asc(system_role::Column::Sort)
        .order_by_desc(system_role::Column::Id)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.pagination.page_no - 1).await?;
    let page = Page::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn get_role_list_by_status(
    status_list: Option<Vec<CommonStatusEnum>>,
) -> ApiResult<Vec<system_role::Model>> {
    let db = database::get_db_async().await;
    let list = SystemRole::find_perm_with_tenant()
        .await
        .apply_if(status_list, |query, status_list| {
            query.filter(system_role::Column::Status.is_in(status_list))
        })
        .order_by_asc(system_role::Column::Sort)
        .order_by_desc(system_role::Column::Id)
        .all(&db)
        .await?;
    Ok(list)
}

#[transactional]
pub async fn delete_role(id: &str) -> ApiResult<()> {
    // 1. 校验是否可以更新
    validate_role_for_update(id).await?;
    let db = database::get_db_async().await;
    // 2.1 标记删除
    SystemRole::delete_logical_by_id(&db, id).await?;
    // 2.2 删除相关数据
    process_role_deleted(id).await?;
    Ok(())
}

#[transactional]
pub async fn process_role_deleted(role_id: &str) -> ApiResult<()> {
    // 标记删除 UserRole
    system_user_role_service::delete_list_by_role_id(role_id).await?;
    // 标记删除 RoleMenu
    system_role_menu_service::delete_list_by_role_id(role_id).await?;
    Ok(())
}

#[transactional]
pub async fn delete_role_list(ids: &Vec<String>) -> ApiResult<()> {
    // 1. 校验是否可以删除
    try_join_all(ids.iter().map(|id| validate_role_for_update(id))).await?;
    // 2.1 标记删除
    let db = database::get_db_async().await;
    SystemRole::batch_delete_logical_by_id(&db, ids).await?;
    // 2.2 删除相关数据
    try_join_all(ids.iter().map(|id| process_role_deleted(id))).await?;
    Ok(())
}

pub async fn update_role_data_scope(vo: PermissionAssignRoleDataScopeReqVO) -> ApiResult<()> {
    // 校验是否可以更新
    let mut active_model = validate_role_for_update(&vo.role_id)
        .await?
        .into_active_model();
    // 更新数据范围
    active_model.data_scope = Set(vo.data_scope);
    active_model.data_scope_dept_ids = Set(vo.data_scope_dept_ids.unwrap_or_default());
    active_model.update(&database::get_db_async().await).await?;
    Ok(())
}
