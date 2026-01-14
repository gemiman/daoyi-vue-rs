use crate::system_entity::prelude::*;
use crate::system_entity::{system_mail_account, system_mail_log, system_mail_template};
use daoyi_common_support::database;
use daoyi_common_support::enumeration::{MailSendStatusEnum, UserTypeEnum};
use daoyi_common_support::error::ApiResult;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::collections::{HashMap, HashSet};

pub async fn create_mail_log(
    user_id: &str,
    user_type: UserTypeEnum,
    to_mails: &HashSet<String>,
    cc_mails: &HashSet<String>,
    bcc_mails: &HashSet<String>,
    account: &system_mail_account::Model,
    template: &system_mail_template::Model,
    template_content: &str,
    template_params: &HashMap<String, String>,
    is_send: bool,
) -> ApiResult<system_mail_log::Model> {
    let db = database::get_db_async().await;
    let active_model = system_mail_log::ActiveModel {
        user_id: Set(String::from(user_id)),
        user_type: Set(user_type),
        to_mails: Set(to_mails.iter().map(|s| s.to_string()).collect()),
        cc_mails: Set(cc_mails.iter().map(|s| s.to_string()).collect()),
        bcc_mails: Set(bcc_mails.iter().map(|s| s.to_string()).collect()),
        account_id: Set(account.id.clone()),
        from_mail: Set(account.mail.clone()),
        template_id: Set(template.id.clone()),
        template_code: Set(template.code.clone()),
        template_nickname: Set(template.nickname.clone()),
        template_title: Set(template.title.clone()),
        template_content: Set(template_content.to_string()),
        template_params: Set(serde_json::to_value(template_params)?),
        send_status: Set(if is_send {
            MailSendStatusEnum::INIT
        } else {
            MailSendStatusEnum::IGNORE
        }),
        ..Default::default()
    };
    Ok(active_model.insert(&db).await?)
}

pub async fn update_mail_log_send_result(
    log_id: &str,
    send_status: MailSendStatusEnum,
    send_message_id: Option<String>,
    send_exception: Option<String>,
) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let log = SystemMailLog::find_by_id(log_id)
        .one(&db)
        .await?
        .ok_or_else(|| daoyi_common_support::error::ApiError::biz("邮件日志不存在"))?;

    let mut active_model: system_mail_log::ActiveModel = log.into();
    active_model.send_status = Set(send_status);
    active_model.send_time = Set(Some(Local::now().naive_local()));
    active_model.send_message_id = Set(send_message_id);
    active_model.send_exception = Set(send_exception);
    active_model.update(&db).await?;
    Ok(())
}
