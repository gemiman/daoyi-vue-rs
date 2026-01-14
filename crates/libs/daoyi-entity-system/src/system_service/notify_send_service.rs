use crate::system_entity::system_notify_template;
use crate::system_service::{system_notify_message_service, system_notify_template_service};
use daoyi_common_support::enumeration::{CommonStatusEnum, UserTypeEnum};
use daoyi_common_support::error::{ApiError, ApiResult};
use std::collections::HashMap;

pub async fn send_single_notify_to_member(
    user_id: &str,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    send_single_notify(
        user_id,
        UserTypeEnum::Member,
        template_code,
        template_params,
    )
    .await
}

pub async fn send_single_notify_to_admin(
    user_id: &str,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    send_single_notify(user_id, UserTypeEnum::Admin, template_code, template_params).await
}

pub async fn send_single_notify(
    user_id: &str,
    user_type: UserTypeEnum,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    // 校验模版
    let template = validate_notify_template(template_code).await?;
    if template.status == CommonStatusEnum::Disable {
        return Err(ApiError::biz(format!(
            "模版({template_code})已经关闭，无法给用户({user_id}/{user_type:?})发送"
        )));
    }
    // 校验参数
    let template_params = validate_template_params(&template, template_params).await?;
    // 发送站内信
    let content = system_notify_template_service::format_notify_template_content(
        &template.content,
        &template_params,
    )
    .await?;
    Ok(system_notify_message_service::create_notify_message(
        String::from(user_id),
        user_type,
        template,
        content,
        template_params,
    )
    .await?
    .id)
}

pub async fn validate_template_params(
    template: &system_notify_template::Model,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<HashMap<String, String>> {
    if template.params.is_empty() {
        return Ok(HashMap::new());
    }
    if template_params.is_none() {
        return Err(ApiError::biz(format!(
            "模版({})需要参数，但未提供",
            template.code
        )));
    }
    let template_params = template_params.as_ref().unwrap();
    for key in template.params.iter() {
        if !template_params.contains_key(key) {
            return Err(ApiError::biz(format!("模板参数({key})缺失",)));
        }
    }
    Ok(template_params.clone())
}

pub async fn validate_notify_template(
    template_code: &str,
) -> ApiResult<system_notify_template::Model> {
    let model = system_notify_template_service::get_notify_template_by_code(template_code)
        .await?
        .ok_or_else(|| ApiError::biz("当前通知公告不存在"))?;
    Ok(model)
}
