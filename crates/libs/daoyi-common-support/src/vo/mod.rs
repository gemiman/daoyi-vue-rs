use crate::context::HttpRequestContext;
use crate::enumeration::ID_ROOT;
use serde::{Deserialize, Serialize};

pub mod cti_vo;
pub mod infra_vo;
pub mod system_vo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MqMsgBody<T> {
    pub topic: String,
    pub token: Option<String>,
    pub tenant_id: Option<String>,
    pub payload: T,
}

impl<T> MqMsgBody<T> {
    pub fn build_with_token_with_tenant<S: Into<String>>(topic: S, payload: T) -> Self {
        Self::new(topic, payload)
            .with_token(
                HttpRequestContext::get_token()
                    .as_deref()
                    .unwrap_or(ID_ROOT),
            )
            .with_tenant_id(
                HttpRequestContext::get_tenant_id()
                    .as_deref()
                    .unwrap_or(ID_ROOT),
            )
    }
    pub fn new<S: Into<String>>(topic: S, payload: T) -> Self {
        Self {
            topic: topic.into(),
            token: None,
            tenant_id: None,
            payload,
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn with_tenant_id(mut self, tenant_id: &str) -> Self {
        self.tenant_id = Some(tenant_id.to_string());
        self
    }
}

use crate::response::ApiResponse;

pub type R<T> = ApiResponse<T>;
