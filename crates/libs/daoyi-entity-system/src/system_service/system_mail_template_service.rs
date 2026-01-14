use crate::system_entity::prelude::*;
use crate::system_entity::system_mail_template;
use daoyi_common_support::database;
use daoyi_common_support::error::ApiResult;
use sea_orm::prelude::*;

pub async fn get_mail_template_count_by_account_id(account_id: &str) -> ApiResult<u64> {
    let db = database::get_db_async().await;
    let count = SystemMailTemplate::find_perm_with_tenant()
        .await
        .filter(system_mail_template::Column::AccountId.eq(account_id))
        .count(&db)
        .await?;
    Ok(count)
}

pub async fn get_mail_template_count_by_account_ids(account_ids: &Vec<String>) -> ApiResult<u64> {
    let db = database::get_db_async().await;
    let count = SystemMailTemplate::find_perm_with_tenant()
        .await
        .filter(system_mail_template::Column::AccountId.is_in(account_ids))
        .count(&db)
        .await?;
    Ok(count)
}
