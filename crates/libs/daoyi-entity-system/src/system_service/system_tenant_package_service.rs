use crate::system_entity::prelude::*;
use crate::system_entity::system_tenant_package;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use sea_orm::entity::prelude::*;

pub async fn valid_tenant_package(id: &str) -> ApiResult<system_tenant_package::Model> {
    let db = database::get().await;
    let model = SystemTenantPackage::find_by_id(id)
        .filter(system_tenant_package::Column::Deleted.eq(false))
        .one(db)
        .await?;
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
    let db = database::get().await;
    let list = SystemTenantPackage::find()
        .filter(system_tenant_package::Column::Deleted.eq(false))
        .filter(system_tenant_package::Column::Status.eq(status))
        .all(db)
        .await?;
    Ok(list)
}
