use crate::system_entity::prelude::*;
use crate::system_entity::system_role;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
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
