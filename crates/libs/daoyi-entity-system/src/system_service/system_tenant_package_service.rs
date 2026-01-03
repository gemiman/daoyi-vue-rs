use crate::system_entity::prelude::*;
use crate::system_entity::system_tenant_package;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::vo::system_vo::TenantPackagePageReqVO;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

pub async fn get_tenant_package(id: &str) -> ApiResult<Option<system_tenant_package::Model>> {
    let db = database::get_db_async().await;
    let model = SystemTenantPackage::find_by_id(id)
        .filter(system_tenant_package::Column::Deleted.eq(false))
        .one(&db)
        .await?;
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
