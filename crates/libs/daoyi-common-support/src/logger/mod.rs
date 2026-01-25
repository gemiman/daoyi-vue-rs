use crate::configs::AppConfig;
use crate::context::HttpRequestContext;
use crate::enumeration::ID_ROOT;
use crate::enumeration::redis_keys::{OPERATE_LOG_GROUP_NAME, OPERATE_LOG_STREAM_KEY};
use crate::error::{ApiError, ApiResult};
use crate::response::ApiResponse;
use crate::vo::MqMsgBody;
use crate::vo::system_vo::operate_log_vo::OperateLogCreateReqDTO;
use crate::{id_util, redis_utils};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod builder;
pub use builder::OperateLogBuilder;

pub async fn init_logger() {
    let app_config = AppConfig::get();
    let log_config = app_config.log();

    // 根据配置创建文件 appender
    let filename = log_config.filename().unwrap_or(app_config.app_name());
    let filename = if !filename.ends_with(".log") {
        format!("{}.log", filename)
    } else {
        filename.to_string()
    };
    let file_appender = match log_config.rolling() {
        "hourly" => rolling::hourly(log_config.dir(), filename),
        "minutely" => rolling::minutely(log_config.dir(), filename),
        "never" => rolling::never(log_config.dir(), filename),
        _ => rolling::daily(log_config.dir(), filename), // 默认按天分割
    };

    // 创建非阻塞写入器
    let (non_blocking_file, _guard) = non_blocking(file_appender);

    // 控制台输出层
    let console_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%Y-%m-%d %H:%M:%S%.3f".to_string(),
        ))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false);

    // 文件输出层
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_file)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%Y-%m-%d %H:%M:%S%.3f".to_string(),
        ))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .with_ansi(false); // 文件输出不需要颜色代码

    let mut log_level =
        std::env::var("RUST_LOG").unwrap_or_else(|_| log_config.level().to_string());
    log_level.push_str(",nacos=info,h2=info,hyper=info,tower=info,lettre=info,rustls=info");

    tracing_subscriber::registry()
        .with(EnvFilter::new(log_level))
        .with(console_layer)
        .with(file_layer)
        .init();

    // 注意：_guard 需要保持存活，否则日志会丢失
    // 可以考虑将其存储在全局变量中
    std::mem::forget(_guard);
}

/// 记录操作日志（异步、优雅的封装）
///
/// # 示例
///
/// ```rust
/// use daoyi_common_support::logger;
///
/// logger::record_operate_log(
///     "订单模块",
///     "创建订单",
///     "1024",
///     "用户创建了订单",
///     None
/// ).await?;
/// ```
///
/// # 参数
/// * `r#type` - 操作模块类型
/// * `sub_type` - 操作名
/// * `biz_id` - 业务 ID
/// * `action` - 操作内容
/// * `extra` - 额外信息
pub async fn record_operate_log(
    r#type: &str,
    sub_type: &str,
    biz_id: &str,
    action: &str,
    extra: Option<serde_json::Value>,
) -> ApiResult<()> {
    let trace_id = HttpRequestContext::get_tracing_id_as_string();
    let user_id = HttpRequestContext::get_login_id_as_string()?;
    let user_type = HttpRequestContext::get_user_type();
    let user_ip = HttpRequestContext::get_user_ip_as_string();
    let user_agent = HttpRequestContext::get_user_agent_as_string();
    let request_method = HttpRequestContext::get_request_method_as_string();
    let request_url = HttpRequestContext::get_request_url_as_string();
    let tenant_id = HttpRequestContext::get_tenant_id_as_string()?;

    let req = OperateLogCreateReqDTO {
        trace_id,
        user_id,
        user_type,
        r#type: r#type.to_string(),
        sub_type: sub_type.to_string(),
        biz_id: biz_id.to_string(),
        action: action.to_string(),
        extra: extra.unwrap_or(serde_json::json!({})),
        request_method,
        request_url,
        user_ip,
        user_agent,
        tenant_id,
    };
    let mq_msg = MqMsgBody::build_with_token_with_tenant(OPERATE_LOG_STREAM_KEY, req);
    match redis_utils::send_mq_msg(&mq_msg).await {
        Ok(msg_id) => {
            tracing::info!("发送日志消息到Redis Stream成功 msg_id: {}", msg_id);
        }
        Err(e) => {
            tracing::error!("发送日志消息到Redis Stream失败， error: {}", e);
        }
    }
    Ok(())
}

/// 初始化操作日志订阅器（在 System 模块启动时调用）
pub async fn init_operate_log_subscriber() -> ApiResult<()> {
    tracing::info!("Initializing operate log subscriber...");
    tokio::spawn(async {
        let consumer_name = format!("consumer-{}", id_util::xid());
        tracing::info!(
            "启动操作日志队列监听(Stream): {}, Consumer: {}",
            OPERATE_LOG_STREAM_KEY,
            consumer_name
        );

        let result = redis_utils::consume_stream(
            OPERATE_LOG_STREAM_KEY,
            OPERATE_LOG_GROUP_NAME,
            &consumer_name,
            |msg_payload| async move {
                match serde_json::from_str::<MqMsgBody<OperateLogCreateReqDTO>>(&msg_payload) {
                    Ok(msg) => {
                        if let Err(e) = create_operate_log(msg).await {
                            tracing::error!("操作日志发送失败 error: {}", e);
                            // 注意：即使发送失败，我们这里返回 Ok，以便 ACK 该消息，防止无限重试。
                            // 实际业务中，可能需要根据错误类型决定是否抛出 Err 触发重试，或者记录到死信队列。
                            // 这里我们已经在 do_send_mail 中更新了数据库状态为"失败"，所以认为任务已终结。
                            return Ok(());
                        }
                        Ok(())
                    }
                    Err(e) => {
                        tracing::error!("解析操作日志消息失败: {}, payload: {}", e, msg_payload);
                        // 解析失败的消息无法处理，直接确认跳过
                        Ok(())
                    }
                }
            },
        )
        .await;

        if let Err(e) = result {
            tracing::error!("操作日志队列监听异常退出: {}", e);
        }
    });
    Ok(())
}

async fn create_operate_log(msg: MqMsgBody<OperateLogCreateReqDTO>) -> ApiResult<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(AppConfig::get().log().log_server_url())
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
        .map_err(|e| ApiError::biz(format!("日志发送失败0: {}", e)))?;
    if !resp.status().is_success() {
        return Err(ApiError::biz(format!(
            "日志发送失败1：status: {}",
            resp.status()
        )));
    }
    let api_response = resp
        .json::<ApiResponse<Option<String>>>()
        .await
        .map_err(|e| ApiError::biz(format!("日志发送失败2: {}", e)))?;
    if !api_response.success {
        return Err(ApiError::biz(api_response.msg));
    }
    Ok(())
}
