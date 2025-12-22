use axum::Router;
use daoyi_common_support::app::AppState;

pub mod user;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/user", user::create_router())
}
