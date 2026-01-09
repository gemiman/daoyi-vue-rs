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
mod notify;
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
        .nest("/notify-message", notify::notify_message::create_router())
        .nest("/notify-template", notify::notify_template::create_router())
        .nest("/oauth2", oauth2::create_router())
        .nest("/permission", permission::permission::create_router())
        .nest("/menu", permission::menu::create_router())
        .nest("/role", permission::role::create_router())
        .nest("/sms", sms::create_router())
        .nest("/social", social::create_router())
        .nest("/tenant", tenant::tenant::create_router())
        .nest("/tenant-package", tenant::tenant_package::create_router())
        .nest("/user", user::user::create_router())
        .nest("/user/profile", user::user_profile::create_router())
}
