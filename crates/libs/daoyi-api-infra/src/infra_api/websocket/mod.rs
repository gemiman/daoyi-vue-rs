use daoyi_common_support::enumeration::UserTypeEnum;
use daoyi_common_support::websocket::WebSocketMessageSender;
use serde::Serialize;
use std::sync::Arc;

/// WebSocket 发送 API (对标 Java WebSocketSenderApi)
#[allow(dead_code)]
pub struct WebSocketSenderApi {
    sender: Arc<WebSocketMessageSender>,
}

impl WebSocketSenderApi {
    #[allow(dead_code)]
    pub fn new(sender: Arc<WebSocketMessageSender>) -> Self {
        Self { sender }
    }

    /// 发送消息给指定用户
    #[allow(dead_code)]
    pub async fn send<T: Serialize>(
        &self,
        user_type: UserTypeEnum,
        user_id: &str,
        msg_type: &str,
        content: T,
    ) {
        self.sender
            .send_by_user(user_type, user_id, msg_type, content)
            .await;
    }

    /// 发送消息给指定类型的全体用户
    #[allow(dead_code)]
    pub async fn send_all_by_user_type<T: Serialize>(
        &self,
        user_type: UserTypeEnum,
        msg_type: &str,
        content: T,
    ) {
        self.sender
            .send_by_user_type(user_type, msg_type, content)
            .await;
    }
}
