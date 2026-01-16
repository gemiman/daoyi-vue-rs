use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsSendRespDTO {
    pub success: bool,
    pub serial_no: Option<String>,
    pub api_request_id: Option<String>,
    pub api_code: Option<String>,
    pub api_msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsReceiveRespDTO {
    pub success: bool,
    pub error_code: Option<String>,
    pub error_msg: Option<String>,
    pub mobile: String,
    pub receive_time: Option<chrono::NaiveDateTime>,
    pub serial_no: Option<String>,
    pub log_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsTemplateRespDTO {
    pub id: String,
    pub content: String,
    pub audit_status: i32, // See SmsTemplateAuditStatusEnum
    pub audit_reason: Option<String>,
}
