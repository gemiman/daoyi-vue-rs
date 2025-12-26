use crate::system_entity::prelude::*;
use crate::system_entity::system_user_role;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use sea_orm::prelude::*;
use std::collections::HashSet;

pub async fn get_user_role_id_list_by_user_id(user_id: &str) -> ApiResult<Vec<String>> {
    let db = database::get().await;
    let list = SystemUserRole::find_perm()
        .await
        .filter(system_user_role::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .into_iter()
        .map(|item| item.role_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    Ok(list)
}
