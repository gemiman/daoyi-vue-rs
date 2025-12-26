use crate::system_entity::prelude::*;
use crate::system_entity::system_users;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use sea_orm::entity::prelude::*;

pub async fn get_by_username(username: &str) -> ApiResult<Option<system_users::Model>> {
    let db = database::get().await;
    let option = SystemUsers::find_perm()
        .await
        .filter(system_users::Column::Username.eq(username))
        .one(db)
        .await?;
    Ok(option)
}

pub async fn get_by_id(id: &str) -> ApiResult<system_users::Model> {
    let db = database::get().await;
    SystemUsers::find_perm()
        .await
        .filter(system_users::Column::Id.eq(id))
        .one(db)
        .await?
        .ok_or(ApiError::biz("用户不存在"))
}
