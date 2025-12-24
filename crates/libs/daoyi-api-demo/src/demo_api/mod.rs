use axum::Router;
use daoyi_common_support::app::AppState;
use daoyi_common_support::middlewares::jwt_auth_layer::get_auth_layer;

pub mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::create_router())
        .route_layer(get_auth_layer())
}
