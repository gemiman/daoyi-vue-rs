use axum::Router;
use daoyi_common_support::app::AppState;

mod auth;
mod captcha;
mod dept;
mod dict_data;
mod dict_type;
mod ip;
mod logger;
mod mail;
mod notice;
mod notify_message;
mod notify_template;
mod oauth2;
mod permission;
mod sms;
mod social;
mod tenant;
mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::create_router())
        .nest("/captcha", captcha::create_router())
        .nest("/dept", dept::create_router())
        .nest("/dict-data", dict_data::create_router())
        .nest("/dict-type", dict_type::create_router())
        .nest("/ip", ip::create_router())
        .nest("/logger", logger::create_router())
        .nest("/mail", mail::create_router())
        .nest("/notice", notice::create_router())
        .nest("/notify-message", notify_message::create_router())
        .nest("/notify-template", notify_template::create_router())
        .nest("/oauth2", oauth2::create_router())
        .nest("/permission", permission::create_router())
        .nest("/sms", sms::create_router())
        .nest("/social", social::create_router())
        .nest("/tenant", tenant::create_router())
        .nest("/user", user::create_router())
}
