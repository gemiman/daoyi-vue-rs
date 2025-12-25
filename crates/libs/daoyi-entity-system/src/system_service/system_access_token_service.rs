use crate::system_entity::prelude::*;
use crate::system_entity::system_access_token;
use daoyi_common_support::database;
use sea_orm::entity::prelude::*;

pub async fn get_access_token(token: &str) -> anyhow::Result<system_access_token::Model> {
    let db = database::get().await;
    let option = SystemAccessToken::find()
        .filter(system_access_token::Column::AccessToken.eq(token))
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No system access token"))?;
    Ok(option)
}
