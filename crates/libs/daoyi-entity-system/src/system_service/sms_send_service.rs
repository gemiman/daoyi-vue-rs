use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::sms::core::client::{SmsClient, SmsClientFactory};
use daoyi_common_support::sms::core::client_factory::SmsClientFactoryImpl;
use daoyi_common_support::sms::core::property::SmsChannelProperties;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

static SMS_CLIENT_FACTORY: OnceLock<SmsClientFactoryImpl> = OnceLock::new();

fn get_factory() -> &'static SmsClientFactoryImpl {
    SMS_CLIENT_FACTORY.get_or_init(SmsClientFactoryImpl::new)
}

pub async fn send_single_sms_to_admin(
    mobile: &str,
    _user_id: Option<&str>,
    template_code: &str,
    template_params: &HashMap<String, String>,
) -> ApiResult<String> {
    // 1. Get Template
    let template = super::system_sms_template_service::get_sms_template_by_code(template_code)
        .await?
        .ok_or_else(|| ApiError::biz(format!("短信模板({})不存在", template_code)))?;

    // 2. Get Client
    let client = get_sms_client(&template.channel_id).await?;

    // 3. Send
    let result = client
        .send_sms(
            0, // TODO: Create Log ID logic and save log before sending
            mobile,
            &template.api_template_id,
            template_params,
        )
        .await?;

    // TODO: Update log status

    Ok(result.serial_no.unwrap_or_default())
}

async fn get_sms_client(channel_id: &str) -> ApiResult<Arc<dyn SmsClient>> {
    let factory = get_factory();
    if let Some(client) = factory.get_sms_client(channel_id) {
        return Ok(client);
    }

    // Load from DB
    let channel = super::system_sms_channel_service::get_sms_channel(channel_id)
        .await?
        .ok_or_else(|| ApiError::biz("短信渠道不存在"))?;

    let properties = SmsChannelProperties {
        id: channel.id.clone(),
        code: channel.code,
        api_key: channel.api_key,
        api_secret: channel.api_secret,
        signature: Some(channel.signature),
        callback_url: channel.callback_url,
    };

    let client = factory.create_or_update_sms_client(properties);
    Ok(client)
}
