use crate::infra_api::ws::handler::handle_socket;
use axum::extract::{Query, State, ws::WebSocketUpgrade};
use axum::response::IntoResponse;
use daoyi_common_support::app::AppState;
use daoyi_common_support::auth::check_token;
use daoyi_common_support::error::ApiError;
use serde::Deserialize;

pub mod demo;
pub mod handler;

#[derive(Debug, Deserialize)]
pub struct WsQuery {
    pub token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // 自动注册 Demo 监听器
    {
        let count = state.ws_manager.get_listeners_count().await;
        if count == 0 {
            state
                .ws_manager
                .add_listener(Box::new(demo::DemoWebSocketMessageListener {
                    sender: state.ws_sender.clone(),
                }))
                .await;
        }
    }

    match check_token(&query.token).await {
        Ok(login_user) => {
            ws.on_upgrade(move |socket| handle_socket(socket, state.ws_manager, login_user))
        }
        Err(_) => ApiError::unauthenticated("WebSocket 鉴权失败").into_response(),
    }
}
