use crate::system_entity::prelude::*;
use crate::system_entity::system_notify_template;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    NotifyTemplatePageReqVO, NotifyTemplateRespVo, NotifyTemplateSaveReqVO,
    NotifyTemplateUpdateReqVO,
};
use sea_orm::prelude::*;
use sea_orm::{QueryOrder, QueryTrait};

async fn validate_notify_template_code_duplicate(id: Option<&str>, code: &str) -> ApiResult<()> {
    if let Some(model) = get_notify_template_by_code(code).await? {
        if id.is_none() || id != Some(&model.id) {
            return Err(ApiError::biz(format!(
                "已经存在编码为【{code}】的站内信模板"
            )));
        }
    }
    Ok(())
}

pub async fn get_notify_template_by_code(
    code: &str,
) -> ApiResult<Option<system_notify_template::Model>> {
    Ok(SystemNotifyTemplate::find_perm_with_tenant()
        .await
        .filter(system_notify_template::Column::Code.eq(code))
        .one(&database::get_db_async().await)
        .await?)
}

pub async fn create_notify_template(
    vo: NotifyTemplateSaveReqVO,
) -> ApiResult<system_notify_template::Model> {
    // 校验站内信编码是否重复
    validate_notify_template_code_duplicate(None, &vo.code).await?;
    // 插入
    let active_model: system_notify_template::ActiveModel = vo.into();
    let db = database::get_db_async().await;
    Ok(active_model.insert(&db).await?)
}

async fn validate_notify_template_exists(id: &str) -> ApiResult<system_notify_template::Model> {
    get_notify_template(id)
        .await?
        .ok_or_else(|| ApiError::biz("站内信模版不存在"))
}

pub async fn update_notify_template(vo: NotifyTemplateUpdateReqVO) -> ApiResult<()> {
    // 校验存在
    validate_notify_template_exists(&vo.id).await?;
    // 校验站内信编码是否重复
    validate_notify_template_code_duplicate(Some(&vo.id), &vo.code).await?;
    // 更新
    let active_model: system_notify_template::ActiveModel = vo.into();
    active_model.update(&database::get_db_async().await).await?;
    Ok(())
}

pub async fn delete_notify_template(id: &str) -> ApiResult<()> {
    // 校验存在
    validate_notify_template_exists(id).await?;
    // 删除
    SystemNotifyTemplate::delete_logical_by_id(&database::get_db_async().await, id).await?;
    Ok(())
}

pub async fn delete_notify_template_list(ids: &Vec<String>) -> ApiResult<()> {
    SystemNotifyTemplate::delete_logical_by_ids(&database::get_db_async().await, ids).await?;
    Ok(())
}

pub async fn get_notify_template(id: &str) -> ApiResult<Option<system_notify_template::Model>> {
    Ok(
        SystemNotifyTemplate::find_by_id_perm_with_tenant(&database::get_db_async().await, id)
            .await?,
    )
}

pub async fn get_notify_template_page(
    params: &NotifyTemplatePageReqVO,
) -> ApiResult<PageResult<NotifyTemplateRespVo>> {
    let db = database::get_db_async().await;
    let paginator = SystemNotifyTemplate::find_perm_with_tenant()
        .await
        .apply_if(params.code.as_ref(), |query, code| {
            query.filter(system_notify_template::Column::Code.contains(code))
        })
        .apply_if(params.name.as_ref(), |query, name| {
            query.filter(system_notify_template::Column::Name.contains(name))
        })
        .apply_if(params.status, |query, status| {
            query.filter(system_notify_template::Column::Status.eq(status))
        })
        .apply_if(params.create_time.as_ref(), |query, create_time| {
            query.filter(
                system_notify_template::Column::CreateTime.between(create_time[0], create_time[1]),
            )
        })
        .order_by_desc(system_notify_template::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(|item| item.into())
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}
