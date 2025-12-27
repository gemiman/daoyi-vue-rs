use crate::enumeration::CommonStatusEnum;
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantRespVO {
    pub id: String,
    pub name: String,
    pub contact_user_id: Option<String>,
    pub contact_name: String,
    pub contact_mobile: Option<String>,
    pub status: CommonStatusEnum,
    pub websites: Option<String>,
    pub package_id: String,
    pub expire_time: DateTime,
    pub account_count: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DictDataSimpleRespVO {
    pub dict_type: String,
    pub value: String,
    pub label: String,
    pub color_type: Option<String>,
    pub css_class: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthPermissionInfoRespVO {
    pub user: UserVO,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub menus: Vec<MenuVO>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserVO {
    pub id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub dept_id: Option<String>,
    pub username: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuVO {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub path: Option<String>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub icon: Option<String>,
    pub visible: bool,
    pub keep_alive: bool,
    pub always_show: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MenuVO>,
}
