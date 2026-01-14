use crate::system_entity::{system_mail_account, system_mail_template};
use crate::system_service::{
    system_mail_account_service, system_mail_log_service, system_mail_template_service,
    system_users_service,
};
use daoyi_common_support::enumeration::{CommonStatusEnum, MailSendStatusEnum, UserTypeEnum};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::utils::templates;
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::collections::{HashMap, HashSet};
use tracing::{error, info};

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
        &content,
        &template_params,
        is_send,
    )
    .await?
    .id;

    if is_send {
        // 4. 发送 MQ 消息，异步执行发送短信 (这里使用 tokio::spawn 模拟异步发送)
        let log_id = log_id.clone();
        tokio::spawn(async move {
            if let Err(e) = do_send_mail(
                &log_id,
                &account,
                &template.nickname,
                &to_mail_set,
                &cc_mail_set,
                &bcc_mail_set,
                &title,
                &content,
            )
            .await
            {
                error!("邮件发送失败，log_id: {}, error: {}", log_id, e);
            }
        });
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

async fn do_send_mail(
    log_id: &str,
    account: &system_mail_account::Model,
    nickname: &Option<String>,
    to_mails: &HashSet<String>,
    cc_mails: &HashSet<String>,
    bcc_mails: &HashSet<String>,
    title: &str,
    content: &str,
) -> ApiResult<()> {
    // 构造发送者
    let from_mailbox = if let Some(nick) = nickname {
        format!("{} <{}>", nick, account.mail)
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("发送者格式错误: {}", e)))?
    } else {
        account
            .mail
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("发送者邮箱格式错误: {}", e)))?
    };

    let mut builder = Message::builder()
        .from(from_mailbox)
        .subject(title)
        .header(header::ContentType::TEXT_HTML);

    for to in to_mails {
        let to_mailbox = to
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("收件人格式错误: {}", e)))?;
        builder = builder.to(to_mailbox);
    }
    for cc in cc_mails {
        let cc_mailbox = cc
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("抄送人格式错误: {}", e)))?;
        builder = builder.cc(cc_mailbox);
    }
    for bcc in bcc_mails {
        let bcc_mailbox = bcc
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("密送人格式错误: {}", e)))?;
        builder = builder.bcc(bcc_mailbox);
    }

    let email = builder
        .body(content.to_string())
        .map_err(|e| ApiError::biz(format!("邮件构建失败: {}", e)))?;

    // 构造 Transport
    let creds = Credentials::new(account.username.clone(), account.password_plaintext.clone());

    let mut transport_builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&account.host)
        .map_err(|e| ApiError::biz(format!("SMTP Host 错误: {}", e)))?
        .port(account.port as u16)
        .credentials(creds);

    if account.ssl_enable {
        transport_builder = transport_builder.tls(Tls::Wrapper(
            TlsParameters::new(account.host.clone())
                .map_err(|e| ApiError::biz(format!("TLS 配置错误: {}", e)))?,
        ));
    } else if account.starttls_enable {
        transport_builder = transport_builder.tls(Tls::Required(
            TlsParameters::new(account.host.clone())
                .map_err(|e| ApiError::biz(format!("TLS 配置错误: {}", e)))?,
        ));
    } else {
        transport_builder = transport_builder.tls(Tls::None);
    }

    let mailer = transport_builder.build();

    // 发送
    match mailer.send(email).await {
        Ok(resp) => {
            info!("邮件发送成功: {:?}", resp);
            // 更新日志为成功 "1"
            let msg_id = resp
                .message()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            system_mail_log_service::update_mail_log_send_result(
                log_id,
                MailSendStatusEnum::SUCCESS,
                Some(msg_id),
                None,
            )
            .await?;
        }
        Err(e) => {
            error!("邮件发送失败: {:?}", e);
            // 更新日志为失败 "2"
            system_mail_log_service::update_mail_log_send_result(
                log_id,
                MailSendStatusEnum::FAILURE,
                None,
                Some(e.to_string()),
            )
            .await?;
        }
    }

    Ok(())
}
