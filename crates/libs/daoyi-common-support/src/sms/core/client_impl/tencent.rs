use crate::error::{ApiError, ApiResult};
use crate::sms::core::client::SmsClient;
use crate::sms::core::dto::{SmsReceiveRespDTO, SmsSendRespDTO, SmsTemplateRespDTO};
use crate::sms::core::property::SmsChannelProperties;
use async_trait::async_trait;
use chrono::Utc;
use hex;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct TencentSmsClient {
    properties: SmsChannelProperties,
    client: reqwest::Client,
}

impl TencentSmsClient {
    pub fn new(properties: SmsChannelProperties) -> Self {
        Self {
            properties,
            client: reqwest::Client::new(),
        }
    }

    fn get_app_id_and_key(&self) -> (String, String) {
        let parts: Vec<&str> = self.properties.api_key.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[1].to_string(), parts[0].to_string()) // "secretId sdkAppId" -> (sdkAppId, secretId)
        } else {
            ("".to_string(), self.properties.api_key.clone())
        }
    }
}

#[async_trait]
impl SmsClient for TencentSmsClient {
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
        let host = "sms.tencentcloudapi.com";
        let service = "sms";
        let action = "SendSms";
        let version = "2021-01-11";
        let region = "ap-guangzhou";
        let now = Utc::now();
        let timestamp = now.timestamp();
        let date = now.format("%Y-%m-%d").to_string();

        let (app_id, secret_id) = self.get_app_id_and_key();
        let secret_key = self.properties.api_secret.as_deref().unwrap_or_default();

        let params_vec: Vec<String> = template_params.values().cloned().collect();

        let body = json!({
            "PhoneNumberSet": [mobile],
            "SmsSdkAppId": app_id,
            "SignName": self.properties.signature,
            "TemplateId": api_template_id,
            "TemplateParamSet": params_vec,
            "SessionContext": log_id.to_string(),
        });

        let payload = body.to_string();

        // 1. Canonical Request
        let canonical_headers = format!(
            "content-type:application/json; charset=utf-8\nhost:{}\nx-tc-action:{}\n",
            host,
            action.to_lowercase()
        );
        let signed_headers = "content-type;host;x-tc-action";
        let hashed_request_payload = hex::encode(Sha256::digest(payload.as_bytes()));

        let canonical_request = format!(
            "POST\n/\n\n{}\n{}\n{}",
            canonical_headers, signed_headers, hashed_request_payload
        );

        // 2. String to Sign
        let credential_scope = format!("{}/{}/tc3_request", date, service);
        let hashed_canonical_request = hex::encode(Sha256::digest(canonical_request.as_bytes()));
        let string_to_sign = format!(
            "TC3-HMAC-SHA256\n{}\n{}\n{}",
            timestamp, credential_scope, hashed_canonical_request
        );

        // 3. Signature
        type HmacSha256 = Hmac<Sha256>;
        let k_date = HmacSha256::new_from_slice(format!("TC3{}", secret_key).as_bytes()).unwrap();
        let mut k_date = k_date.clone();
        k_date.update(date.as_bytes());
        let k_date_res = k_date.finalize().into_bytes();

        let mut k_service = HmacSha256::new_from_slice(&k_date_res).unwrap();
        k_service.update(service.as_bytes());
        let k_service_res = k_service.finalize().into_bytes();

        let mut k_signing = HmacSha256::new_from_slice(&k_service_res).unwrap();
        k_signing.update("tc3_request".as_bytes());
        let k_signing_res = k_signing.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&k_signing_res).unwrap();
        mac.update(string_to_sign.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // Corrected string escape using raw string literal
        let auth_header = format!(
            r"TC3-HMAC-SHA256 Credential={}/{}\, SignedHeaders={}\, Signature={}",
            secret_id, credential_scope, signed_headers, signature
        );

        let url = format!("https://{}", host);
        let mut req_headers = HeaderMap::new();
        req_headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        req_headers.insert("Host", HeaderValue::from_static(host));
        req_headers.insert("X-TC-Action", HeaderValue::from_str(action).unwrap());
        req_headers.insert("X-TC-Version", HeaderValue::from_static(version));
        req_headers.insert(
            "X-TC-Timestamp",
            HeaderValue::from_str(&timestamp.to_string()).unwrap(),
        );
        req_headers.insert("X-TC-Region", HeaderValue::from_static(region));
        req_headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth_header).unwrap(),
        );

        let res = self
            .client
            .post(&url)
            .headers(req_headers)
            .body(payload)
            .send()
            .await
            .map_err(|e| ApiError::biz(format!("Tencent SendSms request failed: {}", e)))?;

        let body_text = res
            .text()
            .await
            .map_err(|e| ApiError::biz(format!("Tencent SendSms read body failed: {}", e)))?;
        let response: serde_json::Value = serde_json::from_str(&body_text).unwrap_or(json!({}));

        let resp = response["Response"].clone();
        if let Some(error) = resp["Error"].as_object() {
            return Ok(SmsSendRespDTO {
                success: false,
                serial_no: None,
                api_request_id: resp["RequestId"].as_str().map(|s| s.to_string()),
                api_code: error["Code"].as_str().map(|s| s.to_string()),
                api_msg: error["Message"].as_str().map(|s| s.to_string()),
            });
        }

        let send_status_set = resp["SendStatusSet"].as_array();
        if let Some(statuses) = send_status_set {
            if let Some(status) = statuses.get(0) {
                return Ok(SmsSendRespDTO {
                    success: status["Code"] == "Ok",
                    serial_no: status["SerialNo"].as_str().map(|s| s.to_string()),
                    api_request_id: resp["RequestId"].as_str().map(|s| s.to_string()),
                    api_code: status["Code"].as_str().map(|s| s.to_string()),
                    api_msg: status["Message"].as_str().map(|s| s.to_string()),
                });
            }
        }

        Err(ApiError::biz("Tencent SendSms response format error"))
    }

    async fn parse_sms_receive_status(&self, text: &str) -> ApiResult<Vec<SmsReceiveRespDTO>> {
        let statuses: Vec<serde_json::Value> = serde_json::from_str(text).unwrap_or_default();
        let mut result = Vec::new();
        for status in statuses {
            result.push(SmsReceiveRespDTO {
                success: status["report_status"] == "SUCCESS",
                error_code: status["errmsg"].as_str().map(|s| s.to_string()),
                error_msg: status["description"].as_str().map(|s| s.to_string()),
                mobile: status["mobile"].as_str().unwrap_or_default().to_string(),
                receive_time: None,
                serial_no: status["sid"].as_str().map(|s| s.to_string()),
                log_id: None,
            });
        }
        Ok(result)
    }

    async fn get_sms_template(&self, _api_template_id: &str) -> ApiResult<SmsTemplateRespDTO> {
        Err(ApiError::biz("Tencent get_sms_template not implemented"))
    }
}
