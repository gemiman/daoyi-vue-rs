use crate::enumeration::SmsChannelEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsChannelProperties {
    pub id: String,           // Database ID
    pub code: SmsChannelEnum, // Channel Code (ALIYUN, etc.)
    pub api_key: String,
    pub api_secret: Option<String>,
    pub signature: Option<String>,
    pub callback_url: Option<String>,
    // Add other properties as needed from Java's SmsChannelProperties
}
