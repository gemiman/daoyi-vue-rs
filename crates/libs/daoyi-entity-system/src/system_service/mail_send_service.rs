use crate::system_entity::{system_mail_account, system_mail_template};
use crate::system_service::{
    system_mail_account_service, system_mail_log_service, system_mail_template_service,
    system_users_service,
};
use daoyi_common_support::enumeration::{
    CommonStatusEnum, ID_ROOT, MailSendStatusEnum, UserTypeEnum,
};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::utils::templates;
use daoyi_common_support::{mail_server, redis_utils};

use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::enumeration::redis_keys::MAIL_SEND_STREAM_KEY;
use daoyi_common_support::vo::MqMsgBody;
use daoyi_common_support::vo::system_vo::MailSendMessage;
use std::collections::{HashMap, HashSet};

pub async fn send_single_mail_to_admin(
    user_id: &str,
    to_mails: &Vec<String>,
    cc_mails: &Vec<String>,
    bcc_mails: &Vec<String>,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    send_single_mail(
        to_mails,
        cc_mails,
        bcc_mails,
        user_id,
        UserTypeEnum::Admin,
        template_code,
        template_params,
    )
    .await
}

pub async fn send_single_mail_to_member(
    user_id: &str,
    to_mails: &Vec<String>,
    cc_mails: &Vec<String>,
    bcc_mails: &Vec<String>,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    send_single_mail(
        to_mails,
        cc_mails,
        bcc_mails,
        user_id,
        UserTypeEnum::Member,
        template_code,
        template_params,
    )
    .await
}

async fn validate_mail_template(template_code: &str) -> ApiResult<system_mail_template::Model> {
    system_mail_template_service::get_mail_template_by_code(template_code)
        .await?
        .ok_or_else(|| ApiError::biz(format!("邮件模版({})不存在", template_code)))
}
async fn validate_mail_account(account_id: &str) -> ApiResult<system_mail_account::Model> {
    system_mail_account_service::get_mail_account(account_id)
        .await?
        .ok_or_else(|| ApiError::biz("邮箱账号不存在"))
}

async fn get_user_mail(user_id: &str, _user_type: UserTypeEnum) -> ApiResult<Option<String>> {
    let user = system_users_service::get_by_id(user_id).await?;
    Ok(user.email)
}

pub async fn send_single_mail(
    to_mails: &Vec<String>,
    cc_mails: &Vec<String>,
    bcc_mails: &Vec<String>,
    user_id: &str,
    user_type: UserTypeEnum,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    // 1.1 校验邮箱模版是否合法
    let template = validate_mail_template(template_code).await?;
    // 1.2 校验邮箱账号是否合法
    let account = validate_mail_account(&template.account_id).await?;
    // 1.3 校验邮件参数是否缺失
    let template_params = validate_params(&template, template_params).await?;

    // 2. 组装邮箱
    let user_mail = get_user_mail(user_id, user_type).await?;
    let mut to_mail_set: HashSet<String> = HashSet::new();
    let mut cc_mail_set: HashSet<String> = HashSet::new();
    let mut bcc_mail_set: HashSet<String> = HashSet::new();
    if let Some(user_mail) = user_mail
        && validator::ValidateEmail::validate_email(&user_mail)
    {
        to_mail_set.insert(user_mail);
    }
    for to in to_mails {
        if validator::ValidateEmail::validate_email(to) {
            to_mail_set.insert(to.clone());
        }
    }
    for cc in cc_mails {
        if validator::ValidateEmail::validate_email(cc) {
            cc_mail_set.insert(cc.clone());
        }
    }
    for bcc in bcc_mails {
        if validator::ValidateEmail::validate_email(bcc) {
            bcc_mail_set.insert(bcc.clone());
        }
    }
    if to_mail_set.is_empty() {
        return Err(ApiError::biz("邮箱不存在"));
    }

    // 3. 创建发送日志。如果模板被禁用，则不发送短信，只记录日志
    let is_send = template.status == CommonStatusEnum::Enable;
    let title = templates::format_template_content(&template.title, &template_params).await?;
    let content = templates::format_template_content(&template.content, &template_params).await?;

    let log_id = system_mail_log_service::create_mail_log(
        user_id,
        user_type,
        &to_mail_set,
        &cc_mail_set,
        &bcc_mail_set,
        &account,
        &template,
        &title,
        &content,
        &template_params,
        is_send,
    )
    .await?
    .id;

    if is_send {
        // 4. 发送 MQ 消息，异步执行发送邮件 (基于 Redis Stream)
        let message = MailSendMessage {
            log_id: Some(log_id.clone()),
            account: account.into(),
            nickname: template.nickname,
            to_mails: to_mail_set,
            cc_mails: cc_mail_set,
            bcc_mails: bcc_mail_set,
            title,
            content,
        };

        let mq_msg = MqMsgBody::new(MAIL_SEND_STREAM_KEY, message)
            .with_token(
                HttpRequestContext::get_token()
                    .as_deref()
                    .unwrap_or(ID_ROOT),
            )
            .with_tenant_id(
                HttpRequestContext::get_tenant_id()
                    .as_deref()
                    .unwrap_or(ID_ROOT),
            );
        match redis_utils::send_mq_msg(&mq_msg).await {
            Ok(msg_id) => {
                tracing::info!(
                    "发送邮件消息到Redis Stream成功，log_id: {}, msg_id: {}",
                    log_id,
                    msg_id
                );
            }
            Err(e) => {
                tracing::error!(
                    "发送邮件消息到Redis Stream失败，log_id: {}, error: {}",
                    log_id,
                    e
                );
            }
        }
    }

    Ok(log_id)
}

async fn validate_params(
    template: &system_mail_template::Model,
    params: &Option<HashMap<String, String>>,
) -> ApiResult<HashMap<String, String>> {
    if template.params.is_empty() {
        return Ok(HashMap::new());
    }
    match params {
        Some(p) => {
            for key in &template.params {
                if !p.contains_key(key) {
                    return Err(ApiError::biz(format!("模板参数({})缺失", key)));
                }
            }
            Ok(p.clone())
        }
        None => Err(ApiError::biz("参数缺失")),
    }
}

pub async fn do_send_mail(msg: MailSendMessage) -> ApiResult<Option<String>> {
    let log_id = msg.log_id.clone();
    let result = mail_server::send_mail(msg).await;
    // 发送
    match result {
        Ok(msg_id) => {
            tracing::info!("{log_id:?} 邮件发送成功: {msg_id:?}");
            if let Some(log_id) = log_id.as_deref() {
                system_mail_log_service::update_mail_log_send_result(
                    log_id,
                    MailSendStatusEnum::SUCCESS,
                    Some(msg_id),
                    None,
                )
                .await?;
            }
        }
        Err(e) => {
            tracing::error!("{log_id:?} 邮件发送失败: {e:?}");
            if let Some(log_id) = log_id.as_deref() {
                system_mail_log_service::update_mail_log_send_result(
                    log_id,
                    MailSendStatusEnum::FAILURE,
                    None,
                    Some(e.to_string()),
                )
                .await?;
            }
        }
    }
    Ok(log_id)
}
