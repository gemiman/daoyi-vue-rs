use crate::error::{ApiError, ApiResult};
use crate::sms::core::client::SmsClient;
use crate::sms::core::dto::{SmsReceiveRespDTO, SmsSendRespDTO, SmsTemplateRespDTO};
use crate::sms::core::property::SmsChannelProperties;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use hex;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct HuaweiSmsClient {
    properties: SmsChannelProperties,
    client: reqwest::Client,
}

impl HuaweiSmsClient {
    pub fn new(properties: SmsChannelProperties) -> Self {
        Self {
            properties,
            client: reqwest::Client::new(),
        }
    }

    fn get_app_key_and_sender(&self) -> (String, String) {
        let parts: Vec<&str> = self.properties.api_key.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[0].to_string(), parts[1].to_string()) // "appKey sender"
        } else {
            (self.properties.api_key.clone(), "".to_string())
        }
    }
}

#[async_trait]
impl SmsClient for HuaweiSmsClient {
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
        let host = "smsapi.cn-north-4.myhuaweicloud.com:443";
        let url = "https://smsapi.cn-north-4.myhuaweicloud.com:443/sms/batchSendSms/v1";
        let now = Utc::now();
        let sdk_date = now.format("%Y%m%dT%H%M%SZ").to_string();

        let (app_key, sender) = self.get_app_key_and_sender();
        let api_secret = self.properties.api_secret.as_deref().unwrap_or_default();

        let mut body_params = HashMap::new();
        body_params.insert("from", sender);
        body_params.insert("to", mobile.to_string());
        body_params.insert("templateId", api_template_id.to_string());

        let params_vec: Vec<String> = template_params.values().cloned().collect();
        // Huawei expects templateParas as JSON array string
        let template_paras_json = serde_json::to_string(&params_vec).unwrap_or_default();
        body_params.insert("templateParas", template_paras_json);

        if let Some(cb) = &self.properties.callback_url {
            body_params.insert("statusCallback", cb.clone());
        } else {
            // Must have key present? Java code appends it.
            body_params.insert("statusCallback", "".to_string());
        }
        body_params.insert("extend", log_id.to_string());

        // Construct body string for signature and request
        // let mut body_str = String::new();
        // The order in Java appendToBody suggests: from, to, templateId, templateParas, statusCallback, extend
        // However, Java uses appendToBody which just appends to a StringBuilder.
        // It does NOT use a Map to build the body string for signature.
        // It builds the string manually.
        // So we must replicate the order EXACTLY.

        // Java:
        // appendToBody(requestBody, "from=", getSender());
        // appendToBody(requestBody, "&to=", mobile);
        // appendToBody(requestBody, "&templateId=", apiTemplateId);
        // appendToBody(requestBody, "&templateParas=", JsonUtils.toJsonString(...));
        // appendToBody(requestBody, "&statusCallback=", properties.getCallbackUrl());
        // appendToBody(requestBody, "&extend=", String.valueOf(sendLogId));

        fn append(sb: &mut String, key: &str, value: &str) {
            if !value.is_empty() {
                sb.push_str(key);
                sb.push_str(&urlencoding::encode(value));
            }
        }

        let mut request_body = String::new();
        append(&mut request_body, "from=", body_params.get("from").unwrap());
        append(&mut request_body, "&to=", body_params.get("to").unwrap());
        append(
            &mut request_body,
            "&templateId=",
            body_params.get("templateId").unwrap(),
        );
        append(
            &mut request_body,
            "&templateParas=",
            body_params.get("templateParas").unwrap(),
        );
        append(
            &mut request_body,
            "&statusCallback=",
            body_params.get("statusCallback").unwrap(),
        );
        append(
            &mut request_body,
            "&extend=",
            body_params.get("extend").unwrap(),
        );

        // 1. Signature
        let signed_headers = "content-type;host;x-sdk-date";
        let canonical_request = format!(
            "POST\n/sms/batchSendSms/v1\n\ncontent-type:application/x-www-form-urlencoded\nhost:{}\nx-sdk-date:{}\n\n{}\n{}",
            host,
            sdk_date,
            signed_headers,
            hex::encode(Sha256::digest(request_body.as_bytes()))
        );

        let string_to_sign = format!(
            "SDK-HMAC-SHA256\n{}\n{}",
            sdk_date,
            hex::encode(Sha256::digest(canonical_request.as_bytes()))
        );

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).unwrap();
        mac.update(string_to_sign.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let auth_header = format!(
            "SDK-HMAC-SHA256 Access={}, SignedHeaders={}, Signature={}",
            app_key, signed_headers, signature
        );

        let mut req_headers = HeaderMap::new();
        req_headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        req_headers.insert("Host", HeaderValue::from_static(host)); // 'host' (lowercase in signature, but usually Case Insensitive in HTTP, but safer to match)
        req_headers.insert("X-Sdk-Date", HeaderValue::from_str(&sdk_date).unwrap());
        req_headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth_header).unwrap(),
        );

        let res = self
            .client
            .post(url)
            .headers(req_headers)
            .body(request_body) // Send the exact string we signed
            .send()
            .await
            .map_err(|e| ApiError::biz(format!("Huawei SendSms request failed: {}", e)))?;

        let body_text = res
            .text()
            .await
            .map_err(|e| ApiError::biz(format!("Huawei SendSms read body failed: {}", e)))?;
        let response: serde_json::Value = serde_json::from_str(&body_text).unwrap_or(json!({}));

        // result check
        if response.get("result").is_none() {
            return Ok(SmsSendRespDTO {
                success: false,
                serial_no: None,
                api_request_id: None,
                api_code: response["code"].as_str().map(|s| s.to_string()),
                api_msg: response["description"].as_str().map(|s| s.to_string()),
            });
        }

        let results = response["result"].as_array();
        if let Some(res_arr) = results {
            if let Some(first) = res_arr.get(0) {
                return Ok(SmsSendRespDTO {
                    success: response["code"] == "000000",
                    serial_no: first["smsMsgId"].as_str().map(|s| s.to_string()),
                    api_request_id: None,
                    api_code: first["status"].as_str().map(|s| s.to_string()), // status in result item
                    api_msg: None,
                });
            }
        }

        Err(ApiError::biz("Huawei SendSms response format error"))
    }

    async fn parse_sms_receive_status(&self, text: &str) -> ApiResult<Vec<SmsReceiveRespDTO>> {
        // Url encoded params string
        let params: HashMap<String, String> = serde_qs::from_str(text).unwrap_or_default();

        let success = params
            .get("status")
            .map(|s| s == "DELIVRD")
            .unwrap_or(false);
        let receive_time = params
            .get("updateTime")
            .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
            .map(|dt: DateTime<chrono::FixedOffset>| dt.naive_utc());

        Ok(vec![SmsReceiveRespDTO {
            success,
            error_code: params.get("status").cloned(),
            error_msg: params.get("statusDesc").cloned(),
            mobile: params.get("to").cloned().unwrap_or_default(),
            receive_time,
            serial_no: params.get("smsMsgId").cloned(),
            log_id: params.get("extend").and_then(|s| s.parse().ok()),
        }])
    }

    async fn get_sms_template(&self, api_template_id: &str) -> ApiResult<SmsTemplateRespDTO> {
        // Not implemented in Java either for full logic
        Ok(SmsTemplateRespDTO {
            id: api_template_id.to_string(),
            content: "".to_string(),
            audit_status: 1, // Success
            audit_reason: None,
        })
    }
}
