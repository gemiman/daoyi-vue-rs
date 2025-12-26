use axum::Router;
use daoyi_common_support::app::AppState;

mod auth;
mod captcha;
mod dept;
mod dict_data;
mod ip;
mod logger;
mod mail;
mod notice;
mod notify;
mod oauth2;
mod permission;
mod sms;
mod socail;
mod tenant;
mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::create_router())
        .nest("/captcha", captcha::create_router())
        .nest("/dept", dept::create_router())
        .nest("/dict-data", dict_data::create_router())
        .nest("/ip", ip::create_router())
        .nest("/logger", logger::create_router())
        .nest("/mail", mail::create_router())
        .nest("/notice", notice::create_router())
        .nest("/notify", notify::create_router())
        .nest("/oauth2", oauth2::create_router())
        .nest("/permission", permission::create_router())
        .nest("/sms", sms::create_router())
        .nest("/social", socail::create_router())
        .nest("/tenant", tenant::create_router())
        .nest("/user", user::create_router())
}
