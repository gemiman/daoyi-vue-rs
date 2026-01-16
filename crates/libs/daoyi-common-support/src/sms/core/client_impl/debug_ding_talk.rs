use crate::enumeration::SmsTemplateAuditStatusEnum;
use crate::error::ApiResult;
use crate::sms::core::client::SmsClient;
use crate::sms::core::dto::{SmsReceiveRespDTO, SmsSendRespDTO, SmsTemplateRespDTO};
use crate::sms::core::property::SmsChannelProperties;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct DebugDingTalkSmsClient {
    properties: SmsChannelProperties,
}

impl DebugDingTalkSmsClient {
    pub fn new(properties: SmsChannelProperties) -> Self {
        Self { properties }
    }
}

#[async_trait]
impl SmsClient for DebugDingTalkSmsClient {
    fn get_id(&self) -> String {
        self.properties.id.clone()
    }

    async fn send_sms(
        &self,
        log_id: i64,
        mobile: &str,
        api_template_id: &str,
        template_params: &HashMap<String, String>,
    ) -> ApiResult<SmsSendRespDTO> {
        println!(
            "[DebugDingTalkSmsClient] Sending SMS. logId={}, mobile={}, template={}, params={:?}",
            log_id, mobile, api_template_id, template_params
        );
        // Mock response
        Ok(SmsSendRespDTO {
            success: true,
            serial_no: Some("mock_serial".to_string()),
            api_request_id: Some("mock_req_id".to_string()),
            api_code: Some("OK".to_string()),
            api_msg: Some("Success".to_string()),
        })
    }

    async fn parse_sms_receive_status(&self, _text: &str) -> ApiResult<Vec<SmsReceiveRespDTO>> {
        Ok(vec![])
    }

    async fn get_sms_template(&self, api_template_id: &str) -> ApiResult<SmsTemplateRespDTO> {
        Ok(SmsTemplateRespDTO {
            id: api_template_id.to_string(),
            content: "Mock Content".to_string(),
            audit_status: SmsTemplateAuditStatusEnum::CHECKING,
            audit_reason: None,
        })
    }
}
