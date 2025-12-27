use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::response::{ApiResponse, RestApiResult};

pub fn create_router() -> Router<AppState> {
    Router::new().route(
        "/get-unread-count",
        routing::get(get_unread_notify_message_count),
    )
}

#[debug_handler]
async fn get_unread_notify_message_count() -> RestApiResult<u64> {
    ApiResponse::success(0)
}
