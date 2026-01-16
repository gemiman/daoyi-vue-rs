use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmsChannelEnum {
    Aliyun,
    DebugDingTalk,
    Tencent,
    Huawei,
    Qiniu,
}

impl SmsChannelEnum {
    pub fn get_code(&self) -> &str {
        match self {
            SmsChannelEnum::Aliyun => "ALIYUN",
            SmsChannelEnum::DebugDingTalk => "DEBUG_DING_TALK",
            SmsChannelEnum::Tencent => "TENCENT",
            SmsChannelEnum::Huawei => "HUAWEI",
            SmsChannelEnum::Qiniu => "QINIU",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "ALIYUN" => Some(SmsChannelEnum::Aliyun),
            "DEBUG_DING_TALK" => Some(SmsChannelEnum::DebugDingTalk),
            "TENCENT" => Some(SmsChannelEnum::Tencent),
            "HUAWEI" => Some(SmsChannelEnum::Huawei),
            "QINIU" => Some(SmsChannelEnum::Qiniu),
            _ => None,
        }
    }
}
