use crate::system_entity::prelude::*;
use crate::system_entity::system_tenant_package;
use crate::system_service::system_tenant_service;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::vo::system_vo::{
    TenantPackagePageReqVO, TenantPackageSaveReqVo, TenantPackageUpdateReqVo,
};
use daoyi_macros::transactional;
use futures::future::try_join_all;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};
use std::collections::HashSet;

pub async fn get_tenant_package(id: &str) -> ApiResult<Option<system_tenant_package::Model>> {
    let db = database::get_db_async().await;
    let model = SystemTenantPackage::find_by_id(id)
        .filter(system_tenant_package::Column::Deleted.eq(false))
        .one(&db)
        .await?;
    Ok(model)
}

pub async fn valid_tenant_package_exists(id: &str) -> ApiResult<system_tenant_package::Model> {
    let model = get_tenant_package(id).await?;
    let model = match model {
        Some(model) => model,
        None => Err(ApiError::biz("租户套餐不存在"))?,
    };
    Ok(model)
}
pub async fn valid_tenant_package(id: &str) -> ApiResult<system_tenant_package::Model> {
    let model = get_tenant_package(id).await?;
    let model = match model {
        Some(model) => {
            if model.status == CommonStatusEnum::Disable {
                return Err(ApiError::biz(format!(
                    "名字为【{}】的租户套餐已被禁用",
                    model.name
                )))?;
            }
            model
        }
        None => Err(ApiError::biz("租户套餐不存在"))?,
    };
    Ok(model)
}
pub async fn get_tenant_package_list_by_status(
    status: CommonStatusEnum,
) -> ApiResult<Vec<system_tenant_package::Model>> {
    let db = database::get_db_async().await;
    let list = SystemTenantPackage::find()
        .filter(system_tenant_package::Column::Deleted.eq(false))
        .filter(system_tenant_package::Column::Status.eq(status))
        .all(&db)
        .await?;
    Ok(list)
}

pub async fn get_tenant_package_page(
    params: &TenantPackagePageReqVO,
) -> ApiResult<Page<system_tenant_package::Model>> {
    let db = database::get_db_async().await;
    let paginator = SystemTenantPackage::find()
        .filter(system_tenant_package::Column::Deleted.eq(false))
        .apply_if(params.remark.as_ref(), |query, remark| {
            query.filter(system_tenant_package::Column::Remark.contains(remark))
        })
        .apply_if(params.create_time.as_ref(), |query, create_time| {
            query.filter(
                system_tenant_package::Column::CreateTime.between(create_time[0], create_time[1]),
            )
        })
        .apply_if(params.name.as_ref(), |query, name| {
            query.filter(system_tenant_package::Column::Name.contains(name))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_tenant_package::Column::Status.eq(status))
        })
        .order_by_desc(system_tenant_package::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.pagination.page_no - 1).await?;
    let page = Page::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn create_tenant_package(
    vo: TenantPackageSaveReqVo,
) -> ApiResult<system_tenant_package::Model> {
    // 校验套餐名是否重复
    validate_tenant_package_name_unique(None, &vo.name).await?;
    // 插入
    let db = database::get_db_async().await;
    let active_model: system_tenant_package::ActiveModel = vo.into();
    let model = active_model.insert(&db).await?;
    Ok(model)
}

async fn validate_tenant_package_name_unique(id: Option<&str>, name: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let model = SystemTenantPackage::find()
        .filter(system_tenant_package::Column::Deleted.eq(false))
        .filter(system_tenant_package::Column::Name.eq(name))
        .one(&db)
        .await?;
    if model.is_some() {
        if id.is_none() {
            return Err(ApiError::biz("已经存在该名字的租户套餐"));
        }
        if model.unwrap().id != id.unwrap() {
            return Err(ApiError::biz("已经存在该名字的租户套餐"));
        }
    }
    Ok(())
}

#[transactional]
pub async fn update_tenant_package(vo: TenantPackageUpdateReqVo) -> ApiResult<()> {
    // 校验存在
    let model = valid_tenant_package_exists(&vo.id).await?;
    // 校验套餐名是否重复
    validate_tenant_package_name_unique(Some(&vo.id), &vo.name).await?;
    // 更新
    let active_model: system_tenant_package::ActiveModel = vo.into();
    let new_model = active_model.update(&database::get_db_async().await).await?;
    // 如果菜单发生变化，则修改每个租户的菜单
    if !is_menu_ids_equal(&model.menu_ids, &new_model.menu_ids) {
        let tenants = system_tenant_service::get_tenant_list_by_package_id(&new_model.id).await?;
        if !tenants.is_empty() {
            try_join_all(tenants.iter().map(async |tenant| {
                system_tenant_service::update_tenant_role_menu(&tenant.id, &new_model.menu_ids)
                    .await
            }))
            .await?;
        }
    }
    Ok(())
}

/// 比较两个菜单 ID 集合是否相等（忽略顺序）
fn is_menu_ids_equal(old_ids: &Vec<String>, new_ids: &Vec<String>) -> bool {
    let old_ids: HashSet<_> = old_ids.iter().collect();
    let new_ids: HashSet<_> = new_ids.iter().collect();
    // 使用 HashSet 的相等性比较，自动忽略顺序
    old_ids == new_ids
}
