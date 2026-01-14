use crate::system_entity::prelude::*;
use crate::system_entity::system_mail_template;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    MailTemplatePageReqVO, MailTemplateRespVO, MailTemplateSaveReqVO, MailTemplateUpdateReqVO,
};
use sea_orm::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

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

async fn validate_code_unique(id: Option<&str>, code: &str) -> ApiResult<()> {
    if let Some(model) = get_mail_template_by_code(code).await? {
        if id.is_none() || id != Some(&model.id) {
            return Err(ApiError::biz(format!("邮件模版 code({code}) 已存在")));
        }
    }
    Ok(())
}

async fn validate_mail_template_exists(id: &str) -> ApiResult<system_mail_template::Model> {
    get_mail_template(id)
        .await?
        .ok_or_else(|| ApiError::biz("邮件模版不存在"))
}

pub async fn get_mail_template(id: &str) -> ApiResult<Option<system_mail_template::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemMailTemplate::find_by_id_perm_with_tenant(&db, id).await?)
}

pub async fn get_mail_template_by_code(
    code: &str,
) -> ApiResult<Option<system_mail_template::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemMailTemplate::find_perm_with_tenant()
        .await
        .filter(system_mail_template::Column::Code.eq(code))
        .one(&db)
        .await?)
}

pub async fn create_mail_template(
    vo: MailTemplateSaveReqVO,
) -> ApiResult<system_mail_template::Model> {
    // 校验 code 是否唯一
    validate_code_unique(None, &vo.code).await?;
    // 插入
    let db = database::get_db_async().await;
    let active_model: system_mail_template::ActiveModel = vo.into();
    Ok(active_model.insert(&db).await?)
}

pub async fn update_mail_template(vo: MailTemplateUpdateReqVO) -> ApiResult<()> {
    // 校验是否存在
    validate_mail_template_exists(&vo.id).await?;
    // 校验 code 是否唯一
    validate_code_unique(Some(&vo.id), &vo.code).await?;
    // 更新
    let db = database::get_db_async().await;
    let active_model: system_mail_template::ActiveModel = vo.into();
    active_model.update(&db).await?;
    Ok(())
}

pub async fn delete_mail_template(id: &str) -> ApiResult<()> {
    // 校验是否存在
    validate_mail_template_exists(id).await?;
    // 删除
    let db = database::get_db_async().await;
    SystemMailTemplate::delete_logical_by_id(&db, id).await?;
    Ok(())
}

pub async fn delete_mail_template_list(ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;
    SystemMailTemplate::delete_logical_by_ids(&db, ids).await?;
    Ok(())
}

pub async fn get_mail_template_page(
    params: &MailTemplatePageReqVO,
) -> ApiResult<PageResult<MailTemplateRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemMailTemplate::find_perm_with_tenant()
        .await
        .apply_if(params.status, |query, val| {
            query.filter(system_mail_template::Column::Status.eq(val))
        })
        .apply_if(params.code.as_ref(), |query, val| {
            query.filter(system_mail_template::Column::Code.contains(val))
        })
        .apply_if(params.name.as_ref(), |query, val| {
            query.filter(system_mail_template::Column::Name.contains(val))
        })
        .apply_if(params.account_id.as_ref(), |query, val| {
            query.filter(system_mail_template::Column::AccountId.eq(val))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_mail_template::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_mail_template::Column::CreateTime)
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

pub async fn get_mail_template_list() -> ApiResult<Vec<system_mail_template::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemMailTemplate::find_perm_with_tenant()
        .await
        .order_by_desc(system_mail_template::Column::CreateTime)
        .all(&db)
        .await?)
}
