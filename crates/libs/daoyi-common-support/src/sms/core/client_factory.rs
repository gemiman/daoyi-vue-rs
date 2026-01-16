use crate::enumeration::SmsChannelEnum;
use crate::sms::core::client::{SmsClient, SmsClientFactory};
use crate::sms::core::client_impl::aliyun::AliyunSmsClient;
use crate::sms::core::client_impl::debug_ding_talk::DebugDingTalkSmsClient;
use crate::sms::core::client_impl::huawei::HuaweiSmsClient;
use crate::sms::core::client_impl::qiniu::QiniuSmsClient;
use crate::sms::core::client_impl::tencent::TencentSmsClient;
use crate::sms::core::property::SmsChannelProperties;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct SmsClientFactoryImpl {
    channel_id_clients: RwLock<HashMap<String, Arc<dyn SmsClient>>>,
    channel_code_clients: RwLock<HashMap<String, Arc<dyn SmsClient>>>,
}

impl Default for SmsClientFactoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl SmsClientFactoryImpl {
    pub fn new() -> Self {
        Self {
            channel_id_clients: RwLock::new(HashMap::new()),
            channel_code_clients: RwLock::new(HashMap::new()),
        }
    }

    fn create_sms_client(&self, properties: &SmsChannelProperties) -> Arc<dyn SmsClient> {
        match properties.code {
            SmsChannelEnum::Aliyun => Arc::new(AliyunSmsClient::new(properties.clone())),
            SmsChannelEnum::Tencent => Arc::new(TencentSmsClient::new(properties.clone())),
            SmsChannelEnum::Huawei => Arc::new(HuaweiSmsClient::new(properties.clone())),
            SmsChannelEnum::Qiniu => Arc::new(QiniuSmsClient::new(properties.clone())),
            SmsChannelEnum::DebugDingTalk => {
                Arc::new(DebugDingTalkSmsClient::new(properties.clone()))
            } // Stub for others to avoid panic for now, falling back to Debug
              // _ => {
              //     println!("Warning: Channel {:?} not implemented, using DebugDingTalk", channel_enum);
              //     Arc::new(DebugDingTalkSmsClient::new(properties.clone()))
              // }
        }
    }
}

impl SmsClientFactory for SmsClientFactoryImpl {
    fn get_sms_client(&self, channel_id: &str) -> Option<Arc<dyn SmsClient>> {
        self.channel_id_clients
            .read()
            .unwrap()
            .get(channel_id)
            .cloned()
    }

    fn get_sms_client_by_code(&self, channel_code: &str) -> Option<Arc<dyn SmsClient>> {
        self.channel_code_clients
            .read()
            .unwrap()
            .get(channel_code)
            .cloned()
    }

    fn create_or_update_sms_client(&self, properties: SmsChannelProperties) -> Arc<dyn SmsClient> {
        let mut clients = self.channel_id_clients.write().unwrap();
        let client = self.create_sms_client(&properties);
        clients.insert(properties.id.clone(), client.clone());
        client
    }
}
