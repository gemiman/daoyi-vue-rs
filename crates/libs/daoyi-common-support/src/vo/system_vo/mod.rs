use crate::serde::datetime_format;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AuthLoginReqVO {
    #[validate(length(min = 4, max = 16, message = "账号长度为4-16"))]
    pub username: String,
    #[validate(length(min = 4, max = 16, message = "密码长度为4-16"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginRespVO {
    pub tenant_id: String,
    pub user_id: String,
    pub access_token: String,
    #[serde(with = "datetime_format")]
    pub expires_time: DateTime,
}
