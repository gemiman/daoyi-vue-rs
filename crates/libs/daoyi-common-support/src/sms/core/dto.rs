use crate::enumeration::SmsTemplateAuditStatusEnum;
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

/// 短信模板 Response DTO
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsTemplateRespDTO {
    /// 模板编号
    pub id: String,
    /// 短信内容
    pub content: String,
    /// 审核状态
    pub audit_status: SmsTemplateAuditStatusEnum,
    /// 审核未通过的理由
    pub audit_reason: Option<String>,
}
