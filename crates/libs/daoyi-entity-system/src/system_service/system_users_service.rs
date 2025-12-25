use crate::system_entity::prelude::*;
use crate::system_entity::system_users;
use daoyi_common_support::database;
use sea_orm::entity::prelude::*;

pub async fn get_by_username(username: &str) -> anyhow::Result<Option<system_users::Model>> {
    let db = database::get().await;
    let option = SystemUsers::find()
        .filter(system_users::Column::Username.eq(username))
        .one(db)
        .await?;
    Ok(option)
}
