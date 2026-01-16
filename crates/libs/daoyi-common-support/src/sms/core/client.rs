use crate::error::ApiResult;
use crate::sms::core::dto::{SmsReceiveRespDTO, SmsSendRespDTO, SmsTemplateRespDTO};
use crate::sms::core::property::SmsChannelProperties;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait SmsClient: Send + Sync {
    fn get_id(&self) -> String;

    async fn send_sms(
        &self,
        log_id: i64,
        mobile: &str,
        api_template_id: &str,
        template_params: &HashMap<String, String>, // Using Map instead of List<KeyValue> for ease
    ) -> ApiResult<SmsSendRespDTO>;

    async fn parse_sms_receive_status(&self, text: &str) -> ApiResult<Vec<SmsReceiveRespDTO>>;

    async fn get_sms_template(&self, api_template_id: &str) -> ApiResult<SmsTemplateRespDTO>;
}

pub trait SmsClientFactory: Send + Sync {
    fn get_sms_client(&self, channel_id: &str) -> Option<Arc<dyn SmsClient>>;
    fn get_sms_client_by_code(&self, channel_code: &str) -> Option<Arc<dyn SmsClient>>;
    fn create_or_update_sms_client(&self, properties: SmsChannelProperties) -> Arc<dyn SmsClient>;
}
