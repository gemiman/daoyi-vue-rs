use crate::system_entity::prelude::*;
use crate::system_entity::{system_sms_channel, system_sms_template};
use crate::system_service::system_sms_channel_service;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::{CommonStatusEnum, SmsTemplateAuditStatusEnum};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{
    SmsTemplatePageReqVO, SmsTemplateRespVO, SmsTemplateSaveReqVO, SmsTemplateUpdateReqVO,
};
use sea_orm::prelude::*;
use sea_orm::{QueryOrder, QueryTrait, Set};

pub async fn create_sms_template(
    vo: SmsTemplateSaveReqVO,
) -> ApiResult<system_sms_template::Model> {
    // 校验短信渠道
    let channel = validate_sms_channel(&vo.channel_id).await?;
    // 校验短信编码是否重复
    validate_sms_template_code_duplicate(None, &vo.code).await?;
    // 校验短信模板
    validate_api_template(&channel.id, &vo.api_template_id).await?;
    let db = database::get_db_async().await;
    let mut active_model: system_sms_template::ActiveModel = vo.into();
    active_model.channel_code = Set(channel.code);
    let result = active_model.insert(&db).await?;
    Ok(result)
}

async fn validate_api_template(channel_id: &str, api_template_id: &str) -> ApiResult<()> {
    // 获得短信模板
    let sms_client = system_sms_channel_service::get_sms_client(channel_id).await?;
    let template = sms_client.get_sms_template(api_template_id).await?;
    if template.audit_status == SmsTemplateAuditStatusEnum::CHECKING {
        return Err(ApiError::biz("短信 API 模版无法使用，原因：审批中"));
    }
    if template.audit_status == SmsTemplateAuditStatusEnum::FAIL {
        return Err(ApiError::biz(format!(
            "短信 API 模版无法使用，原因：审批不通过，{:?}",
            template.audit_reason
        )));
    }
    Ok(())
}

async fn validate_sms_template_code_duplicate(id: Option<&str>, code: &str) -> ApiResult<()> {
    if let Some(model) = get_sms_template_by_code(code).await? {
        if id.is_none() || id != Some(&model.id) {
            return Err(ApiError::biz(format!("已经存在编码为【{code}】的短信模板")));
        }
    }
    Ok(())
}

async fn validate_sms_channel(channel_id: &str) -> ApiResult<system_sms_channel::Model> {
    let model = system_sms_channel_service::get_sms_channel(channel_id)
        .await?
        .ok_or_else(|| ApiError::biz("短信渠道不存在"))?;
    if model.status == CommonStatusEnum::Disable {
        return Err(ApiError::biz("短信渠道不处于开启状态，不允许选择"));
    }
    Ok(model)
}

pub async fn update_sms_template(vo: SmsTemplateUpdateReqVO) -> ApiResult<()> {
    // 校验存在
    validate_sms_template_exists(&vo.id).await?;
    // 校验短信渠道
    let channel = validate_sms_channel(&vo.channel_id).await?;
    // 校验短信编码是否重复
    validate_sms_template_code_duplicate(Some(&vo.id), &vo.code).await?;
    // 校验短信模板
    validate_api_template(&channel.id, &vo.api_template_id).await?;
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
    SystemSmsTemplate::delete_logical_by_id(&db, id).await?;
    Ok(())
}

pub async fn delete_sms_template_list(ids: &Vec<String>) -> ApiResult<()> {
    let db = database::get_db_async().await;
    SystemSmsTemplate::delete_logical_by_ids(&db, ids).await?;
    Ok(())
}

pub async fn get_sms_template(id: &str) -> ApiResult<Option<system_sms_template::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemSmsTemplate::find_by_id_perm_with_tenant(&db, id).await?)
}

pub async fn get_sms_template_by_code(code: &str) -> ApiResult<Option<system_sms_template::Model>> {
    let db = database::get_db_async().await;
    Ok(SystemSmsTemplate::find_perm_with_tenant()
        .await
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
    let paginator = SystemSmsTemplate::find_perm_with_tenant()
        .await
        .apply_if(params.r#type, |query, val| {
            query.filter(system_sms_template::Column::Type.eq(val))
        })
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
            query.filter(system_sms_template::Column::ApiTemplateId.contains(api_template_id))
        })
        .apply_if(params.channel_id.as_ref(), |query, channel_id| {
            query.filter(system_sms_template::Column::ChannelId.eq(channel_id))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(system_sms_channel::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(system_sms_template::Column::CreateTime)
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

pub async fn get_sms_template_count_by_channel_id(channel_id: &str) -> ApiResult<u64> {
    let db = database::get_db_async().await;
    let count = SystemSmsTemplate::find_perm_with_tenant()
        .await
        .filter(system_sms_template::Column::ChannelId.eq(channel_id))
        .count(&db)
        .await?;
    Ok(count)
}

pub async fn get_sms_template_count_by_channel_ids(channel_ids: &Vec<String>) -> ApiResult<u64> {
    let db = database::get_db_async().await;
    let count = SystemSmsTemplate::find_perm_with_tenant()
        .await
        .filter(system_sms_template::Column::ChannelId.is_in(channel_ids))
        .count(&db)
        .await?;
    Ok(count)
}
