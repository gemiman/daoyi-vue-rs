use crate::enumeration::SmsTemplateAuditStatusEnum;
use crate::error::{ApiError, ApiResult};
use crate::sms::core::client::SmsClient;
use crate::sms::core::dto::{SmsReceiveRespDTO, SmsSendRespDTO, SmsTemplateRespDTO};
use crate::sms::core::property::SmsChannelProperties;
use async_trait::async_trait;
use base64::prelude::*;
use chrono::{TimeZone, Utc};
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use sha1::Sha1;
use std::collections::HashMap;

pub struct QiniuSmsClient {
    properties: SmsChannelProperties,
    client: reqwest::Client,
}

impl QiniuSmsClient {
    pub fn new(properties: SmsChannelProperties) -> Self {
        Self {
            properties,
            client: reqwest::Client::new(),
        }
    }

    fn get_signature(&self, method: &str, path: &str, body: Option<&str>, date: &str) -> String {
        let host = "sms.qiniuapi.com";
        let content_type = "application/json";

        let mut data_to_sign = String::new();
        data_to_sign.push_str(&format!(
            "{} {}
Host: {}
Content-Type: {}
X-Qiniu-Date: {}

",
            method.to_uppercase(),
            path,
            host,
            content_type,
            date
        ));

        if let Some(b) = body {
            data_to_sign.push_str(b);
        }

        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(self.properties.api_secret.as_str().as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(data_to_sign.as_bytes());
        let signature = BASE64_STANDARD
            .encode(mac.finalize().into_bytes())
            .replace("+", "-")
            .replace("/", "_"); // URL safe base64? Java uses standard base64?
        // Java: SecureUtil.hmac(HmacAlgorithm.HmacSHA1, ...).digestBase64(..., true) -> UrlSafe is true?
        // Checking Java code: digestBase64(data, true) -> isUrlSafe = true.
        // So we need URL Safe Base64.
        // Or replace + with - and / with _ manually as I did above.

        format!("Qiniu {}:{}", self.properties.api_key, signature)
    }
}

#[async_trait]
impl SmsClient for QiniuSmsClient {
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
        let host = "sms.qiniuapi.com";
        let path = "/v1/message/single";
        let url = format!("https://{}{}", host, path);
        let now = Utc::now();
        let sign_date = now.format("%Y%m%dT%H%M%SZ").to_string();

        let body_map = json!({
            "template_id": api_template_id,
            "mobile": mobile,
            "parameters": template_params,
            "seq": log_id.to_string(),
        });
        let body_str = body_map.to_string();

        let auth_header = self.get_signature("POST", path, Some(&body_str), &sign_date);

        let mut req_headers = HeaderMap::new();
        req_headers.insert("Host", HeaderValue::from_static(host));
        req_headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth_header).unwrap(),
        );
        req_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        req_headers.insert("X-Qiniu-Date", HeaderValue::from_str(&sign_date).unwrap());

        let res = self
            .client
            .post(&url)
            .headers(req_headers)
            .body(body_str)
            .send()
            .await
            .map_err(|e| ApiError::biz(format!("Qiniu SendSms request failed: {}", e)))?;

        let body_text = res
            .text()
            .await
            .map_err(|e| ApiError::biz(format!("Qiniu SendSms read body failed: {}", e)))?;
        let response: serde_json::Value = serde_json::from_str(&body_text).unwrap_or(json!({}));

        if let Some(error) = response.get("error") {
            if !error.as_str().unwrap_or("").is_empty() {
                return Ok(SmsSendRespDTO {
                    success: false,
                    serial_no: None,
                    api_request_id: response["request_id"].as_str().map(|s| s.to_string()),
                    api_code: response["error"].as_str().map(|s| s.to_string()),
                    api_msg: response["message"].as_str().map(|s| s.to_string()),
                });
            }
        }

        Ok(SmsSendRespDTO {
            success: response.get("message_id").is_some(),
            serial_no: response["message_id"].as_str().map(|s| s.to_string()),
            api_request_id: None,
            api_code: None,
            api_msg: None,
        })
    }

    async fn parse_sms_receive_status(&self, text: &str) -> ApiResult<Vec<SmsReceiveRespDTO>> {
        let root: serde_json::Value = serde_json::from_str(text).unwrap_or(json!({}));
        let items = root["items"].as_array();
        let mut result = Vec::new();
        if let Some(item_list) = items {
            for status in item_list {
                let receive_time = status["delivrd_at"]
                    .as_i64()
                    .and_then(|ts| Utc.timestamp_opt(ts, 0).single())
                    .map(|dt: chrono::DateTime<Utc>| dt.naive_utc());

                result.push(SmsReceiveRespDTO {
                    success: status["status"] == "DELIVRD",
                    error_code: None,
                    error_msg: status["status"].as_str().map(|s| s.to_string()),
                    mobile: status["mobile"].as_str().unwrap_or_default().to_string(),
                    receive_time,
                    serial_no: status["message_id"].as_str().map(|s| s.to_string()),
                    log_id: status["seq"].as_i64(),
                });
            }
        }
        Ok(result)
    }

    async fn get_sms_template(&self, api_template_id: &str) -> ApiResult<SmsTemplateRespDTO> {
        let path = format!("/v1/template/{}", api_template_id);
        let url = format!("https://sms.qiniuapi.com{}", path);
        let now = Utc::now();
        let sign_date = now.format("%Y%m%dT%H%M%SZ").to_string();

        let auth_header = self.get_signature("GET", &path, None, &sign_date);

        let mut req_headers = HeaderMap::new();
        req_headers.insert("Host", HeaderValue::from_static("sms.qiniuapi.com"));
        req_headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth_header).unwrap(),
        );
        req_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        req_headers.insert("X-Qiniu-Date", HeaderValue::from_str(&sign_date).unwrap());

        let res = self
            .client
            .get(&url)
            .headers(req_headers)
            .send()
            .await
            .map_err(|e| ApiError::biz(format!("Qiniu GetTemplate request failed: {}", e)))?;

        let body_text = res
            .text()
            .await
            .map_err(|e| ApiError::biz(format!("Qiniu GetTemplate read body failed: {}", e)))?;
        let response: serde_json::Value = serde_json::from_str(&body_text).unwrap_or(json!({}));

        // Parse status
        let audit_status = match response["audit_status"].as_str().unwrap_or("") {
            "passed" => SmsTemplateAuditStatusEnum::SUCCESS, // Success
            "reviewing" => SmsTemplateAuditStatusEnum::CHECKING, // Checking
            "rejected" => SmsTemplateAuditStatusEnum::FAIL,  // Fail
            _ => SmsTemplateAuditStatusEnum::FAIL,           // Default Fail or Unknown
        };

        Ok(SmsTemplateRespDTO {
            id: response["id"].as_str().unwrap_or_default().to_string(),
            content: response["template"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            audit_status,
            audit_reason: response["reject_reason"].as_str().map(|s| s.to_string()),
        })
    }
}
