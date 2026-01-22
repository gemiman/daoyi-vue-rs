use axum::Router;
use daoyi_common_support::app::AppState;

mod auth;
mod captcha;
mod dept;
mod dict;
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
        .nest("/dept", dept::dept::create_router())
        .nest("/post", dept::posts::create_router())
        .nest("/dict-data", dict::dict_data::create_router())
        .nest("/dict-type", dict::dict_type::create_router())
        .nest("/ip", ip::create_router())
        .nest("/login-log", logger::login_log::create_router())
        .nest("/operate-log", logger::operate_log::create_router())
        .nest("/mail-account", mail::mail_account::create_router())
        .nest("/mail-log", mail::mail_log::create_router())
        .nest("/mail-template", mail::mail_template::create_router())
        .nest("/notice", notice::create_router())
        .nest("/notify-message", notify::notify_message::create_router())
        .nest("/notify-template", notify::notify_template::create_router())
        .nest("/oauth2", oauth2::create_router())
        .nest("/permission", permission::permission::create_router())
        .nest("/menu", permission::menu::create_router())
        .nest("/role", permission::role::create_router())
        .nest("/sms-channel", sms::sms_channel::create_channel_router())
        .nest("/sms-template", sms::sms_template::create_template_router())
        .nest("/sms-log", sms::sms_log::create_router())
        .nest("/social", social::create_router())
        .nest("/tenant", tenant::tenant::create_router())
        .nest("/tenant-package", tenant::tenant_package::create_router())
        .nest("/user", user::user::create_router())
        .nest("/user/profile", user::user_profile::create_router())
}
