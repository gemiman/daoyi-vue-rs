use axum::extract::ws::{Message, WebSocket};
use daoyi_common_support::vo::system_vo::AuthLoginRespVO;
use daoyi_common_support::websocket::{WebSocketSession, WebSocketSessionManager};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn handle_socket(
    socket: WebSocket,
    manager: Arc<WebSocketSessionManager>,
    user: AuthLoginRespVO,
) {
    let session_id = xid::new().to_string();
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // 发送任务
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = sender.send(msg).await {
                tracing::error!("WebSocket send error: {}", e);
                break;
            }
        }
    });

    // 创建已认证的 session
    let session = Arc::new(WebSocketSession::new(
        session_id.clone(),
        Some(user.user_id.clone()),
        Some(1), // UserType: Admin
        Some(user.tenant_id),
        tx,
    ));

    manager.add_session(session.clone()).await;
    tracing::info!(
        "WebSocket connection established: {} for user: {}",
        session_id,
        user.user_id
    );

    // 接收循环
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                manager.handle_message(session.clone(), &text).await;
            }
            Message::Close(_) => break,
            Message::Ping(_) => {
                // 回复文本 "pong" 以对标前端 useWebSocket 的 heartbeat
                let _ = session.send(Message::Text("pong".into()));
            }
            _ => {}
        }
    }

    // 清理
    manager.remove_session(&session_id).await;
    send_task.abort();
    tracing::info!("WebSocket connection closed: {}", session_id);
}
