use crate::system_entity::prelude::*;
use crate::system_entity::system_sms_template;
use daoyi_common_support::database;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    SmsTemplatePageReqVO, SmsTemplateRespVO, SmsTemplateSaveReqVO, SmsTemplateUpdateReqVO,
};
use sea_orm::prelude::*;
use sea_orm::{QueryTrait, Set};

pub async fn create_sms_template(vo: SmsTemplateSaveReqVO) -> ApiResult<String> {
    let db = database::get_db_async().await;

    // Check Channel
    let channel = super::system_sms_channel_service::get_sms_channel(&vo.channel_id)
        .await?
        .ok_or_else(|| ApiError::biz("短信渠道不存在"))?;

    let mut active_model: system_sms_template::ActiveModel = vo.into();
    active_model.channel_code = Set(channel.code);

    let result = active_model.insert(&db).await?;
    Ok(result.id)
}

pub async fn update_sms_template(vo: SmsTemplateUpdateReqVO) -> ApiResult<()> {
    // 校验是否存在
    validate_sms_template_exists(&vo.id).await?;

    // Check Channel
    let channel = super::system_sms_channel_service::get_sms_channel(&vo.channel_id)
        .await?
        .ok_or_else(|| ApiError::biz("短信渠道不存在"))?;

    // 更新
    let db = database::get_db_async().await;
    let mut active_model: system_sms_template::ActiveModel = vo.into();
    active_model.channel_code = Set(channel.code);

    active_model.update(&db).await?;
    Ok(())
}

pub async fn delete_sms_template(id: &str) -> ApiResult<()> {
    // 校验是否存在
    validate_sms_template_exists(id).await?;
    // 删除
    let db = database::get_db_async().await;
    SystemSmsTemplate::delete_by_id(id).exec(&db).await?;
    Ok(())
}

pub async fn delete_sms_template_list(ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;
    SystemSmsTemplate::delete_many()
        .filter(system_sms_template::Column::Id.is_in(ids))
        .exec(&db)
        .await?;
    Ok(())
}

pub async fn get_sms_template(id: &str) -> ApiResult<Option<system_sms_template::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemSmsTemplate::find_by_id(id).one(&db).await?)
}

pub async fn get_sms_template_by_code(code: &str) -> ApiResult<Option<system_sms_template::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemSmsTemplate::find()
        .filter(system_sms_template::Column::Code.eq(code))
        .one(&db)
        .await?)
}

async fn validate_sms_template_exists(id: &str) -> ApiResult<system_sms_template::Model> {
    get_sms_template(id)
        .await?
        .ok_or_else(|| ApiError::biz("当前短信模板不存在"))
}

pub async fn get_sms_template_page(
    params: &SmsTemplatePageReqVO,
) -> ApiResult<PageResult<SmsTemplateRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemSmsTemplate::find()
        // .apply_if(params.r#type, |query, type_| {
        //     query.filter(system_sms_template::Column::Type.eq(type_))
        // })
        .apply_if(params.status, |query, status| {
            query.filter(system_sms_template::Column::Status.eq(status))
        })
        .apply_if(params.code.as_ref(), |query, code| {
            query.filter(system_sms_template::Column::Code.contains(code))
        })
        .apply_if(params.content.as_ref(), |query, content| {
            query.filter(system_sms_template::Column::Content.contains(content))
        })
        .apply_if(params.api_template_id.as_ref(), |query, api_template_id| {
            query.filter(system_sms_template::Column::ApiTemplateId.eq(api_template_id))
        })
        .apply_if(params.channel_id.as_ref(), |query, channel_id| {
            query.filter(system_sms_template::Column::ChannelId.eq(channel_id))
        })
        // .order_by_desc(system_sms_template::Column::CreateTime)
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
