use async_trait::async_trait;
use daoyi_common_support::enumeration::UserTypeEnum;
use daoyi_common_support::websocket::{
    WebSocketMessageListener, WebSocketMessageSender, WebSocketSession,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoSendMessage {
    pub to_user_id: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoReceiveMessage {
    pub from_user_id: Option<String>,
    pub text: String,
    pub single: bool,
}

pub struct DemoWebSocketMessageListener {
    pub sender: Arc<WebSocketMessageSender>,
}

#[async_trait]
impl WebSocketMessageListener for DemoWebSocketMessageListener {
    async fn on_message(&self, session: Arc<WebSocketSession>, content: Value) {
        if let Ok(msg) = serde_json::from_value::<DemoSendMessage>(content) {
            let from_user_id = session.user_id.clone();

            if let Some(to_user_id) = msg.to_user_id {
                let to_message = DemoReceiveMessage {
                    from_user_id: from_user_id.clone(),
                    text: msg.text,
                    single: true,
                };
                self.sender
                    .send_by_user(
                        UserTypeEnum::Admin,
                        &to_user_id,
                        "demo-message-receive",
                        to_message,
                    )
                    .await;
            } else {
                let to_message = DemoReceiveMessage {
                    from_user_id: from_user_id.clone(),
                    text: msg.text,
                    single: false,
                };
                self.sender
                    .send_by_user_type(UserTypeEnum::Admin, "demo-message-receive", to_message)
                    .await;
            }
        }
    }

    fn get_type(&self) -> &str {
        "demo-message-send"
    }
}
