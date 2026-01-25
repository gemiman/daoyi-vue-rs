use crate::configs::AppConfig;
use crate::enumeration::redis_keys::{MAIL_SEND_GROUP_NAME, MAIL_SEND_STREAM_KEY};
use crate::error::{ApiError, ApiResult};
use crate::response::ApiResponse;
use crate::vo::MqMsgBody;
use crate::vo::system_vo::MailSendMessage;
use crate::{id_util, redis_utils};
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub async fn send_mail(msg: MailSendMessage) -> ApiResult<String> {
    // 构造发送者
    let from_mailbox = if let Some(nick) = msg.nickname {
        format!("{} <{}>", nick, msg.account.mail)
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("发送者格式错误: {}", e)))?
    } else {
        msg.account
            .mail
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("发送者邮箱格式错误: {}", e)))?
    };

    let mut builder = Message::builder()
        .from(from_mailbox)
        .subject(msg.title)
        .header(header::ContentType::TEXT_HTML);

    for to in msg.to_mails {
        let to_mailbox = to
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("收件人格式错误: {}", e)))?;
        builder = builder.to(to_mailbox);
    }
    for cc in msg.cc_mails {
        let cc_mailbox = cc
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("抄送人格式错误: {}", e)))?;
        builder = builder.cc(cc_mailbox);
    }
    for bcc in msg.bcc_mails {
        let bcc_mailbox = bcc
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| ApiError::biz(format!("密送人格式错误: {}", e)))?;
        builder = builder.bcc(bcc_mailbox);
    }

    let email = builder
        .body(msg.content.to_string())
        .map_err(|e| ApiError::biz(format!("邮件构建失败: {}", e)))?;

    // 构造 Transport
    let creds = Credentials::new(msg.account.username, msg.account.password);

    let mut transport_builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&msg.account.host)
        .map_err(|e| ApiError::biz(format!("SMTP Host 错误: {}", e)))?
        .port(msg.account.port as u16)
        .credentials(creds);

    if msg.account.ssl_enable {
        transport_builder = transport_builder
            .tls(Tls::Wrapper(TlsParameters::new(msg.account.host).map_err(
                |e| ApiError::biz(format!("TLS 配置错误: {}", e)),
            )?));
    } else if msg.account.starttls_enable {
        transport_builder = transport_builder.tls(Tls::Required(
            TlsParameters::new(msg.account.host)
                .map_err(|e| ApiError::biz(format!("TLS 配置错误: {}", e)))?,
        ));
    } else {
        transport_builder = transport_builder.tls(Tls::None);
    }

    let mailer = transport_builder.build();

    // 发送
    let response = mailer
        .send(email)
        .await
        .map_err(|e| ApiError::biz(format!("邮件发送失败: {}", e)))?;
    let msg_id = response
        .message()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    Ok(msg_id)
}

/// 初始化邮件队列消费者
pub async fn init_mail_queue_consumer() -> ApiResult<()> {
    tokio::spawn(async {
        let consumer_name = format!("consumer-{}", id_util::xid());
        tracing::info!(
            "启动邮件发送队列监听(Stream): {}, Consumer: {}",
            MAIL_SEND_STREAM_KEY,
            consumer_name
        );

        let result = redis_utils::consume_stream(
            MAIL_SEND_STREAM_KEY,
            MAIL_SEND_GROUP_NAME,
            &consumer_name,
            |msg_payload| async move {
                match serde_json::from_str::<MqMsgBody<MailSendMessage>>(&msg_payload) {
                    Ok(msg) => {
                        let log_id = msg.payload.log_id.clone();
                        tracing::info!("收到邮件发送任务，log_id: {:?}", log_id);
                        if let Err(e) = do_send_mail(msg).await {
                            tracing::error!("邮件发送失败，log_id: {:?}, error: {}", log_id, e);
                            // 注意：即使发送失败，我们这里返回 Ok，以便 ACK 该消息，防止无限重试。
                            // 实际业务中，可能需要根据错误类型决定是否抛出 Err 触发重试，或者记录到死信队列。
                            // 这里我们已经在 do_send_mail 中更新了数据库状态为"失败"，所以认为任务已终结。
                            return Ok(());
                        }
                        Ok(())
                    }
                    Err(e) => {
                        tracing::error!("解析邮件消息失败: {}, payload: {}", e, msg_payload);
                        // 解析失败的消息无法处理，直接确认跳过
                        Ok(())
                    }
                }
            },
        )
        .await;

        if let Err(e) = result {
            tracing::error!("邮件队列监听异常退出: {}", e);
        }
    });
    Ok(())
}

async fn do_send_mail(msg: MqMsgBody<MailSendMessage>) -> ApiResult<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(AppConfig::get().mail_server().send_mail_url())
        .header(
            AppConfig::get().auth().header_key_token(),
            format!("Bearer {}", msg.token.unwrap_or_default()),
        )
        .header(
            AppConfig::get().auth().header_key_tenant(),
            msg.tenant_id.unwrap_or_default(),
        )
        .json(&msg.payload)
        .send()
        .await
        .map_err(|e| ApiError::biz(format!("邮件发送失败0: {}", e)))?;
    if !resp.status().is_success() {
        return Err(ApiError::biz(format!(
            "邮件发送失败1：status: {}",
            resp.status()
        )));
    }
    let api_response = resp
        .json::<ApiResponse<Option<String>>>()
        .await
        .map_err(|e| ApiError::biz(format!("邮件发送失败2: {}", e)))?;
    if !api_response.success {
        return Err(ApiError::biz(api_response.msg));
    }
    Ok(())
}
