use serde::{Deserialize, Serialize};

pub mod cti_vo;
pub mod infra_vo;
pub mod system_vo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MqMsgBody<T> {
    pub topic: String,
    pub token: Option<String>,
    pub payload: T,
}

impl<T> MqMsgBody<T> {
    pub fn new<S: Into<String>>(topic: S, payload: T) -> Self {
        Self {
            topic: topic.into(),
            token: None,
            payload,
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }
}
