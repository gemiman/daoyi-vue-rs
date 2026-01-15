use crate::configs::AppConfig;
use crate::enumeration::{UserTypeEnum, WEBSOCKET_REDIS_CHANNEL};
use crate::error::ApiResult;
use async_trait::async_trait;
use axum::extract::ws::Message;
use futures::StreamExt;
use redis::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock, mpsc};

static WS_MANAGER: OnceCell<Arc<WebSocketSessionManager>> = OnceCell::const_new();
static WS_SENDER: OnceCell<Arc<WebSocketMessageSender>> = OnceCell::const_new();

async fn init_ws() -> anyhow::Result<()> {
    let ws_manager = WS_MANAGER
        .get_or_try_init(async || {
            let ws_manager = Arc::new(WebSocketSessionManager::new());
            Ok::<_, anyhow::Error>(ws_manager)
        })
        .await?;
    WS_SENDER
        .get_or_try_init(async || {
            let ws_sender = Arc::new(WebSocketMessageSender::new(ws_manager.clone()));
            Ok::<_, anyhow::Error>(ws_sender)
        })
        .await?;
    Ok(())
}

fn get_ws_sender() -> ApiResult<Arc<WebSocketMessageSender>> {
    WS_SENDER
        .get()
        .map(|s| s.clone())
        .ok_or_else(|| anyhow::anyhow!("WebSocket sender not initialized").into())
}

/// JSON 格式的 WebSocket 消息帧
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonWebSocketMessage {
    pub r#type: String,
    /// 消息内容：注意这里为了对标前端 JSON.parse(jsonMessage.content)，必须是字符串
    pub content: String,
}

/// WebSocket Session 包装
pub struct WebSocketSession {
    pub id: String,
    pub user_id: Option<String>,
    pub user_type: Option<UserTypeEnum>,
    pub tenant_id: Option<String>,
    sender: mpsc::UnboundedSender<Message>,
}

impl WebSocketSession {
    pub fn new(
        id: String,
        user_id: Option<String>,
        user_type: Option<UserTypeEnum>,
        tenant_id: Option<String>,
        sender: mpsc::UnboundedSender<Message>,
    ) -> Self {
        Self {
            id,
            user_id,
            user_type,
            tenant_id,
            sender,
        }
    }

    pub fn send(&self, msg: Message) -> anyhow::Result<()> {
        self.sender
            .send(msg)
            .map_err(|e| anyhow::anyhow!("Send failed: {}", e))
    }

    pub async fn send_json<T: Serialize>(&self, msg_type: &str, content: T) -> anyhow::Result<()> {
        let content_json = serde_json::to_string(&content)?;
        let msg = JsonWebSocketMessage {
            r#type: msg_type.to_string(),
            content: content_json,
        };
        let text = serde_json::to_string(&msg)?;
        self.send(Message::Text(text.into()))
    }
}

/// WebSocket 消息监听器 Trait
#[async_trait]
pub trait WebSocketMessageListener: Send + Sync {
    async fn on_message(&self, session: Arc<WebSocketSession>, content: Value);
    fn get_type(&self) -> &str;
}

/// Redis 广播消息
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisWebSocketMessage {
    pub session_id: Option<String>,
    pub user_type: Option<UserTypeEnum>,
    pub user_id: Option<String>,
    pub message_type: String,
    /// Redis 广播时内容保持 Value 即可，发送给客户端前会转为 String
    pub message_content: Value,
}

/// WebSocket Session 管理器
pub struct WebSocketSessionManager {
    sessions: RwLock<HashMap<String, Arc<WebSocketSession>>>,
    listeners: RwLock<HashMap<String, Box<dyn WebSocketMessageListener>>>,
}

impl WebSocketSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            listeners: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_session(&self, session: Arc<WebSocketSession>) {
        self.sessions
            .write()
            .await
            .insert(session.id.clone(), session);
    }

    pub async fn remove_session(&self, id: &str) {
        self.sessions.write().await.remove(id);
    }

    pub async fn add_listener(&self, listener: Box<dyn WebSocketMessageListener>) {
        self.listeners
            .write()
            .await
            .insert(listener.get_type().to_string(), listener);
    }

    pub async fn get_listeners_count(&self) -> usize {
        self.listeners.read().await.len()
    }

    pub async fn handle_message(&self, session: Arc<WebSocketSession>, text: &str) {
        // 特殊处理心跳: 前端可能会发送字符串 "ping"
        if text == "ping" {
            let _ = session.send(Message::Text("pong".into()));
            return;
        }

        if let Ok(msg) = serde_json::from_str::<JsonWebSocketMessage>(text) {
            let listeners = self.listeners.read().await;
            if let Some(listener) = listeners.get(&msg.r#type) {
                // 将 content 字符串解析回 Value 传给 listener
                if let Ok(content_value) = serde_json::from_str::<Value>(&msg.content) {
                    listener.on_message(session, content_value).await;
                }
            } else {
                tracing::info!("No listener found for message type: {}", msg.r#type);
            }
        }
    }

    pub async fn get_session(&self, id: &str) -> Option<Arc<WebSocketSession>> {
        self.sessions.read().await.get(id).cloned()
    }

    pub async fn get_session_list_by_user_type(
        &self,
        user_type: UserTypeEnum,
    ) -> Vec<Arc<WebSocketSession>> {
        self.sessions
            .read()
            .await
            .values()
            .filter(|s| s.user_type == Some(user_type))
            .cloned()
            .collect()
    }

    pub async fn get_session_list_by_user(
        &self,
        user_type: UserTypeEnum,
        user_id: &str,
    ) -> Vec<Arc<WebSocketSession>> {
        self.sessions
            .read()
            .await
            .values()
            .filter(|s| s.user_type == Some(user_type) && s.user_id.as_deref() == Some(user_id))
            .cloned()
            .collect()
    }

    pub async fn get_all_sessions(&self) -> Vec<Arc<WebSocketSession>> {
        self.sessions.read().await.values().cloned().collect()
    }
}

/// WebSocket 消息发送者
pub struct WebSocketMessageSender {
    manager: Arc<WebSocketSessionManager>,
    redis_channel: String,
}

impl WebSocketMessageSender {
    pub fn new(manager: Arc<WebSocketSessionManager>) -> Self {
        Self {
            manager,
            redis_channel: String::from(WEBSOCKET_REDIS_CHANNEL),
        }
    }

    pub async fn send_local(&self, msg: RedisWebSocketMessage) {
        if let Some(sid) = msg.session_id {
            if let Some(session) = self.manager.get_session(&sid).await {
                let _ = session
                    .send_json(&msg.message_type, msg.message_content)
                    .await;
            }
        } else if let (Some(ut), Some(uid)) = (msg.user_type, msg.user_id) {
            let sessions = self.manager.get_session_list_by_user(ut, &uid).await;
            for session in sessions {
                let _ = session
                    .send_json(&msg.message_type, &msg.message_content)
                    .await;
            }
        } else if let Some(ut) = msg.user_type {
            let sessions = self.manager.get_session_list_by_user_type(ut).await;
            for session in sessions {
                let _ = session
                    .send_json(&msg.message_type, &msg.message_content)
                    .await;
            }
        } else {
            let sessions = self.manager.get_all_sessions().await;
            for session in sessions {
                let _ = session
                    .send_json(&msg.message_type, &msg.message_content)
                    .await;
            }
        }
    }

    async fn publish_to_redis(&self, msg: RedisWebSocketMessage) {
        use crate::redis_utils;
        if let Ok(payload) = serde_json::to_string(&msg) {
            let _ = redis_utils::publish(&self.redis_channel, payload).await;
        }
    }

    pub async fn send_by_user<T: Serialize>(
        &self,
        user_type: UserTypeEnum,
        user_id: &str,
        msg_type: &str,
        content: T,
    ) {
        let msg = RedisWebSocketMessage {
            session_id: None,
            user_type: Some(user_type),
            user_id: Some(user_id.to_string()),
            message_type: msg_type.to_string(),
            message_content: serde_json::to_value(content).unwrap_or(Value::Null),
        };
        self.publish_to_redis(msg).await;
    }

    pub async fn send_by_user_type<T: Serialize>(
        &self,
        user_type: UserTypeEnum,
        msg_type: &str,
        content: T,
    ) {
        let msg = RedisWebSocketMessage {
            session_id: None,
            user_type: Some(user_type),
            user_id: None,
            message_type: msg_type.to_string(),
            message_content: serde_json::to_value(content).unwrap_or(Value::Null),
        };
        self.publish_to_redis(msg).await;
    }

    pub async fn send_all<T: Serialize>(&self, msg_type: &str, content: T) {
        let msg = RedisWebSocketMessage {
            session_id: None,
            user_type: None,
            user_id: None,
            message_type: msg_type.to_string(),
            message_content: serde_json::to_value(content).unwrap_or(Value::Null),
        };
        self.publish_to_redis(msg).await;
    }
}

/// 启动 Redis WebSocket 订阅任务
pub async fn start_redis_ws_subscription() -> ApiResult<()> {
    init_ws().await?;
    let ws_sender = get_ws_sender()?;
    tokio::spawn(async move {
        let redis_config = AppConfig::get().redis();
        let host = redis_config.host();
        let port = redis_config.port();
        let db = redis_config.database();
        let passwd = redis_config.password();

        let url = if passwd.is_empty() {
            format!("redis://{host}:{port}/{db}")
        } else {
            format!("redis://:{passwd}@{host}:{port}/{db}")
        };

        let client = match Client::open(url) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Redis Client 初始化失败: {}", e);
                return;
            }
        };

        loop {
            // Redis 1.0 获取异步 PubSub
            match client.get_async_pubsub().await {
                Ok(mut pub_sub) => {
                    if let Err(e) = pub_sub.subscribe(WEBSOCKET_REDIS_CHANNEL).await {
                        tracing::error!("Redis 订阅失败: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }

                    tracing::info!("Redis WebSocket 频道订阅成功: {}", WEBSOCKET_REDIS_CHANNEL);

                    let mut stream = pub_sub.on_message();
                    while let Some(msg) = stream.next().await {
                        let payload_res: redis::RedisResult<String> = msg.get_payload();
                        match payload_res {
                            Ok(payload) => {
                                // 解析消息并分发
                                if let Ok(redis_msg) =
                                    serde_json::from_str::<RedisWebSocketMessage>(&payload)
                                {
                                    ws_sender.send_local(redis_msg).await;
                                }
                            }
                            Err(e) => {
                                tracing::error!("解析 Redis 消息 Payload 失败: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("获取 Redis PubSub 连接失败: {}", e);
                }
            }
            // 发生错误或连接断开后，等待重连
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
    Ok(())
}
