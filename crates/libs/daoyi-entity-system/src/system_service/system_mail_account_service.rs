use crate::system_entity::prelude::*;
use crate::system_entity::system_mail_account;
use crate::system_service::system_mail_template_service;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    MailAccountPageReqVO, MailAccountRespVO, MailAccountSaveReqVO, MailAccountUpdateReqVO,
};
use sea_orm::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

pub async fn create_mail_account(
    vo: MailAccountSaveReqVO,
) -> ApiResult<system_mail_account::Model> {
    let db = database::get_db_async().await;
    let active_model: system_mail_account::ActiveModel = vo.into();
    let model = active_model.insert(&db).await?;
    Ok(model)
}

async fn validate_mail_account_exists(id: &str) -> ApiResult<system_mail_account::Model> {
    get_mail_account(id)
        .await?
        .ok_or_else(|| ApiError::biz("邮箱账号不存在"))
}

pub async fn update_mail_account(vo: MailAccountUpdateReqVO) -> ApiResult<()> {
    // 校验是否存在
    validate_mail_account_exists(&vo.id).await?;
    // 更新
    let db = database::get_db_async().await;
    let active_model: system_mail_account::ActiveModel = vo.into();
    active_model.update(&db).await?;
    Ok(())
}

pub async fn delete_mail_account(id: &str) -> ApiResult<()> {
    // 校验是否存在账号
    validate_mail_account_exists(id).await?;
    // 校验是否存在关联模版
    if system_mail_template_service::get_mail_template_count_by_account_id(id).await? > 0 {
        return Err(ApiError::biz("无法删除，该邮箱账号还有邮件模板"));
    }
    // 删除
    let db = database::get_db_async().await;
    SystemMailAccount::delete_logical_by_id(&db, id).await?;
    Ok(())
}

pub async fn delete_mail_account_list(ids: &Vec<String>) -> ApiResult<()> {
    // 1. 校验是否存在关联模版
    if system_mail_template_service::get_mail_template_count_by_account_ids(ids).await? > 0 {
        return Err(ApiError::biz("无法删除，该邮箱账号还有邮件模板"));
    }
    // 2. 批量删除
    let db = database::get_db_async().await;
    SystemMailAccount::delete_logical_by_ids(&db, ids).await?;
    Ok(())
}

pub async fn get_mail_account(id: &str) -> ApiResult<Option<system_mail_account::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemMailAccount::find_by_id_perm_with_tenant(&db, id).await?)
}

pub async fn get_mail_account_page(
    params: &MailAccountPageReqVO,
) -> ApiResult<PageResult<MailAccountRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemMailAccount::find_perm_with_tenant()
        .await
        .apply_if(params.mail.as_ref(), |query, val| {
            query.filter(system_mail_account::Column::Mail.contains(val))
        })
        .apply_if(params.username.as_ref(), |query, val| {
            query.filter(system_mail_account::Column::Username.contains(val))
        })
        .order_by_desc(system_mail_account::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}

pub async fn get_mail_account_list() -> ApiResult<Vec<system_mail_account::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemMailAccount::find_perm_with_tenant()
        .await
        .all(&db)
        .await?)
}
