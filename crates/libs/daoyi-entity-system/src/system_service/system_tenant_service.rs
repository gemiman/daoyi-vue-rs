use crate::system_entity::prelude::*;
use crate::system_entity::system_tenant;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::enumeration::redis_keys::RedisKey;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::models::system::TenantPageReqVo;
use daoyi_common_support::vo::system_vo::TenantRespVO;
use daoyi_common_support::{database, redis_utils};
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{QueryOrder, QueryTrait};

pub async fn get_tenant_list_by_status(
    status: Option<CommonStatusEnum>,
) -> ApiResult<Vec<system_tenant::Model>> {
    let db = database::get().await;
    let list = SystemTenant::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .apply_if(status, |query, status| {
            query.filter(system_tenant::Column::Status.eq(status))
        })
        .all(db)
        .await?;
    Ok(list)
}
pub async fn get_tenant_by_id(tenant_id: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get().await;
    let option = SystemTenant::find_by_id(tenant_id)
        .filter(system_tenant::Column::Deleted.eq(false))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(option)
}

pub async fn get_tenant_by_name(name: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get().await;
    let option = SystemTenant::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .filter(system_tenant::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::biz("租户不存在"))?;
    Ok(option)
}

pub async fn get_tenant_by_website(website: &str) -> ApiResult<system_tenant::Model> {
    let db = database::get().await;
    let option = SystemTenant::find()
        .filter(system_tenant::Column::Websites.eq(website))
        .filter(system_tenant::Column::Deleted.eq(false))
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

pub async fn get_tenant_page(params: &TenantPageReqVo) -> ApiResult<Page<system_tenant::Model>> {
    let paginator = SystemTenant::find()
        .filter(system_tenant::Column::Deleted.eq(false))
        .apply_if(params.contact_mobile.as_ref(), |query, contact_mobile| {
            query.filter(system_tenant::Column::ContactMobile.contains(contact_mobile))
        })
        .apply_if(params.contact_name.as_ref(), |query, contact_name| {
            query.filter(system_tenant::Column::ContactName.contains(contact_name))
        })
        .apply_if(params.create_time.as_ref(), |query, create_time| {
            query
                .filter(system_tenant::Column::CreateTime.between(&create_time[0], &create_time[1]))
        })
        .apply_if(params.name.as_ref(), |query, name| {
            query.filter(system_tenant::Column::Name.contains(name))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_tenant::Column::Status.eq(status))
        })
        .order_by_desc(system_tenant::Column::CreateTime)
        .paginate(database::get().await, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.pagination.page_no - 1).await?;
    let page = Page::from_pagination(&params.pagination, total, list);
    Ok(page)
}
