use crate::system_entity::prelude::*;
use crate::system_entity::system_tenant_package;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::error::ApiResult;
use sea_orm::entity::prelude::*;

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
