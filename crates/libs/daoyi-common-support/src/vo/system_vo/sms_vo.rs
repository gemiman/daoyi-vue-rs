use crate::enumeration::CommonStatusEnum;
use crate::models::pagination::PaginationParams;
use crate::serde::datetime_format;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;
// ==================== SmsChannel ====================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsChannelPageReqVO {
    pub signature: Option<String>,
    pub status: Option<CommonStatusEnum>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SmsChannelSaveReqVO {
    pub signature: String,
    pub code: String,
    pub status: CommonStatusEnum,
    pub remark: Option<String>,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SmsChannelUpdateReqVO {
    pub id: String,
    pub signature: String,
    pub code: String,
    pub status: CommonStatusEnum,
    pub remark: Option<String>,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmsChannelRespVO {
    pub id: String,
    pub signature: String,
    pub code: String,
    pub status: CommonStatusEnum,
    pub remark: Option<String>,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub callback_url: Option<String>,
    #[serde(with = "datetime_format")]
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmsChannelSimpleRespVO {
    pub id: String,
    pub signature: String,
    pub code: String,
}

// ==================== SmsTemplate ====================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsTemplatePageReqVO {
    pub r#type: Option<i32>, // 0: 验证码, 1: 通知, 2: 营销. Using i32 or Enum. Java uses Integer.
    pub status: Option<CommonStatusEnum>,
    pub code: Option<String>,
    pub content: Option<String>,
    pub api_template_id: Option<String>,
    pub channel_id: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsTemplateSaveReqVO {
    pub r#type: i32,
    pub status: CommonStatusEnum,
    pub code: String,
    pub name: String,
    pub content: String,
    pub remark: Option<String>,
    pub api_template_id: String,
    pub channel_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsTemplateUpdateReqVO {
    pub id: String,
    pub r#type: i32,
    pub status: CommonStatusEnum,
    pub code: String,
    pub name: String,
    pub content: String,
    pub remark: Option<String>,
    pub api_template_id: String,
    pub channel_id: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmsTemplateRespVO {
    pub id: String,
    pub r#type: i32,
    pub status: CommonStatusEnum,
    pub code: String,
    pub name: String,
    pub content: String,
    pub params: Vec<String>,
    pub remark: Option<String>,
    pub api_template_id: String,
    pub channel_id: String,
    pub channel_code: String,
    #[serde(with = "datetime_format")]
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsTemplateSendReqVO {
    pub mobile: String,
    pub template_code: String,
    pub template_params: HashMap<String, String>,
}
