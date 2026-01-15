use crate::system_entity::prelude::*;
use crate::system_entity::{system_mail_account, system_mail_log, system_mail_template};
use daoyi_common_support::database;
use daoyi_common_support::enumeration::{MailSendStatusEnum, UserTypeEnum};
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::system_vo::{MailLogPageReqVO, MailLogRespVO};
use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, PaginatorTrait, QueryOrder, QueryTrait, Set};
use std::collections::{HashMap, HashSet};

pub async fn create_mail_log(
    user_id: &str,
    user_type: UserTypeEnum,
    to_mails: &HashSet<String>,
    cc_mails: &HashSet<String>,
    bcc_mails: &HashSet<String>,
    account: &system_mail_account::Model,
    template: &system_mail_template::Model,
    template_title: &str,
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
        template_title: Set(template_title.to_string()),
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
    let log = SystemMailLog::find_by_id_perm(&db, log_id)
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

pub async fn get_mail_log(id: &str) -> ApiResult<Option<system_mail_log::Model>> {
    let db = database::get_db_async().await;
    let log = SystemMailLog::find_by_id_perm_with_tenant(&db, id).await?;
    Ok(log)
}

pub async fn get_mail_log_page(params: &MailLogPageReqVO) -> ApiResult<PageResult<MailLogRespVO>> {
    let db = database::get_db_async().await;
    let paginator = SystemMailLog::find_perm_with_tenant()
        .await
        .apply_if(params.user_id.as_ref(), |query, val| {
            query.filter(system_mail_log::Column::UserId.eq(val))
        })
        .apply_if(params.user_type, |query, val| {
            query.filter(system_mail_log::Column::UserType.eq(val))
        })
        .apply_if(params.account_id.as_ref(), |query, val| {
            query.filter(system_mail_log::Column::AccountId.eq(val))
        })
        .apply_if(params.template_id.as_ref(), |query, val| {
            query.filter(system_mail_log::Column::TemplateId.eq(val))
        })
        .apply_if(params.send_status, |query, val| {
            query.filter(system_mail_log::Column::SendStatus.eq(val))
        })
        .apply_if(params.send_time.as_ref(), |query, val| {
            query.filter(system_mail_log::Column::SendTime.between(val[0], val[1]))
        })
        .apply_if(params.to_mail.as_ref(), |query, val| {
            query.filter(Expr::cust_with_values("$1 = ANY(to_mails)", [val]))
        })
        .order_by_desc(system_mail_log::Column::CreateTime)
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
