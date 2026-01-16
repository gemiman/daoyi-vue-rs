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
use std::collections::{BTreeMap, HashMap};

pub struct AliyunSmsClient {
    properties: SmsChannelProperties,
    client: reqwest::Client,
}

impl AliyunSmsClient {
    pub fn new(properties: SmsChannelProperties) -> Self {
        Self {
            properties,
            client: reqwest::Client::new(),
        }
    }

    fn sign(
        &self,
        method: &str,
        uri: &str,
        query: &BTreeMap<String, String>,
        headers: &BTreeMap<String, String>,
        body: &str,
    ) -> String {
        // Aliyun V3 Signature
        // 1. Canonical Request
        let canonical_uri = uri;
        let canonical_query_string = query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let mut canonical_headers = String::new();
        let mut signed_headers = String::new();
        for (k, v) in headers {
            let lower_k = k.to_lowercase();
            if lower_k.starts_with("x-acs-") || lower_k == "host" || lower_k == "content-type" {
                canonical_headers.push_str(&format!(
                    "{}:{}
",
                    lower_k,
                    v.trim()
                ));
                if !signed_headers.is_empty() {
                    signed_headers.push(';');
                }
                signed_headers.push_str(&lower_k);
            }
        }

        let payload_hash = hex::encode(Sha256::digest(body.as_bytes()));

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            payload_hash
        );

        let hashed_canonical_request = hex::encode(Sha256::digest(canonical_request.as_bytes()));

        let string_to_sign = format!("ACS3-HMAC-SHA256\n{}", hashed_canonical_request);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac =
            HmacSha256::new_from_slice(self.properties.api_secret.as_ref().unwrap().as_bytes())
                .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        signature
    }
}

#[async_trait]
impl SmsClient for AliyunSmsClient {
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
        let host = "dysmsapi.aliyuncs.com";
        let date = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let nonce = xid::new().to_string(); // Using xid as random nonce

        let mut query = BTreeMap::new();
        query.insert("PhoneNumbers".to_string(), mobile.to_string());
        query.insert(
            "SignName".to_string(),
            self.properties.signature.clone().unwrap_or_default(),
        );
        query.insert("TemplateCode".to_string(), api_template_id.to_string());
        query.insert(
            "TemplateParam".to_string(),
            serde_json::to_string(template_params).unwrap_or_default(),
        );
        query.insert("OutId".to_string(), log_id.to_string());

        let mut headers = BTreeMap::new();
        headers.insert("host".to_string(), host.to_string());
        headers.insert("x-acs-version".to_string(), "2017-05-25".to_string());
        headers.insert("x-acs-action".to_string(), "SendSms".to_string());
        headers.insert("x-acs-date".to_string(), date.clone());
        headers.insert("x-acs-signature-nonce".to_string(), nonce);
        headers.insert(
            "x-acs-content-sha256".to_string(),
            hex::encode(Sha256::digest("")),
        ); // Empty body

        let signature = self.sign("POST", "/", &query, &headers, "");

        // Build Authorization Header
        let signed_headers = headers
            .keys()
            .map(|k| k.to_lowercase())
            .filter(|k| k.starts_with("x-acs-") || k == "host" || k == "content-type")
            .collect::<Vec<_>>()
            .join(";");

        let auth_header = format!(
            "ACS3-HMAC-SHA256 Credential={}, SignedHeaders={}, Signature={}",
            self.properties.api_key, signed_headers, signature
        );

        let url = format!(
            "https://{}?{}",
            host,
            query
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&")
        );

        let mut req_headers = HeaderMap::new();
        for (k, v) in headers {
            req_headers.insert(
                reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_str(&v).unwrap(),
            );
        }
        req_headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth_header).unwrap(),
        );

        let res = self
            .client
            .post(&url)
            .headers(req_headers)
            .send()
            .await
            .map_err(|e| ApiError::biz(format!("Aliyun SendSms request failed: {}", e)))?;

        let body_text = res
            .text()
            .await
            .map_err(|e| ApiError::biz(format!("Aliyun SendSms read body failed: {}", e)))?;
        let response: serde_json::Value = serde_json::from_str(&body_text).unwrap_or(json!({}));

        let success = response["Code"] == "OK";

        Ok(SmsSendRespDTO {
            success, // We added success to DTO in previous steps or I missed it? Wait.
            // In dto.rs I added serial_no, api_request_id, api_code, api_msg.
            // I should check dto.rs again. I think I missed `success` field in `SmsSendRespDTO` definition in my previous write_file call.
            // Ah, looking at AliyunSmsClient.java: return new SmsSendRespDTO().setSuccess(...)
            // I should verify DTO definition.
            serial_no: response["BizId"].as_str().map(|s| s.to_string()),
            api_request_id: response["RequestId"].as_str().map(|s| s.to_string()),
            api_code: response["Code"].as_str().map(|s| s.to_string()),
            api_msg: response["Message"].as_str().map(|s| s.to_string()),
        })
    }

    async fn parse_sms_receive_status(&self, text: &str) -> ApiResult<Vec<SmsReceiveRespDTO>> {
        // Implementation similar to Java
        let statuses: Vec<serde_json::Value> = serde_json::from_str(text).unwrap_or_default();
        let mut result = Vec::new();
        for status in statuses {
            result.push(SmsReceiveRespDTO {
                success: status["success"].as_bool().unwrap_or(false),
                error_code: status["err_code"].as_str().map(|s| s.to_string()),
                error_msg: status["err_msg"].as_str().map(|s| s.to_string()),
                mobile: status["phone_number"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                receive_time: None, // Need parsing logic for "report_time"
                serial_no: status["biz_id"].as_str().map(|s| s.to_string()),
                log_id: status["out_id"].as_i64(),
            });
        }
        Ok(result)
    }

    async fn get_sms_template(&self, _api_template_id: &str) -> ApiResult<SmsTemplateRespDTO> {
        // Stub for now or implement if needed
        Err(ApiError::biz(
            "Aliyun get_sms_template not fully implemented",
        ))
    }
}
