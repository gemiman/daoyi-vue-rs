use crate::enumeration::SmsChannelEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsChannelProperties {
    /// 渠道编号
    pub id: String,
    /// 短信签名
    pub signature: String,
    /// 渠道编码
    pub code: SmsChannelEnum,
    /// 短信 API 的账号
    pub api_key: String,
    /// 短信 API 的密钥
    pub api_secret: String,
    /// 短信发送回调 URL
    pub callback_url: Option<String>,
}
