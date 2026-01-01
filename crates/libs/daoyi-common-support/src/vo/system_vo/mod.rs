use crate::enumeration::{CommonStatusEnum, SexEnum};
use crate::request::validation;
use crate::serde::datetime_format;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// UserSaveReqVO，管理后台 - 用户创建/修改 Request VO
#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserSaveReqVo {
    /// 用户编号
    pub id: Option<String>,
    /// 用户账号
    #[validate(
        length(min = 4, max = 16, message = "账号长度为4-16"),
        custom(function = "validation::is_valid_username")
    )]
    pub username: String,
    /// 用户昵称
    #[validate(length(max = 30, message = "用户昵称长度不能超过30个字符"))]
    pub nickname: String,
    /// 备注
    pub remark: Option<String>,
    /// 部门编号
    pub dept_id: Option<String>,
    /// 岗位编号数组
    pub post_ids: Option<Vec<String>>,
    /// 用户邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    /// 手机号码
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub mobile: Option<String>,
    /// 用户性别，参见 SexEnum 枚举类
    pub sex: Option<SexEnum>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 密码
    #[validate(length(min = 4, max = 16, message = "密码长度为4-16"))]
    pub password: String,
}

impl From<&TenantSaveReqVo> for UserSaveReqVo {
    fn from(value: &TenantSaveReqVo) -> Self {
        Self {
            username: value.username.clone(),
            password: value.password.clone(),
            nickname: value.contact_name.clone(),
            mobile: value.contact_mobile.clone(),
            ..Default::default()
        }
    }
}

/// RoleSaveReqVO，管理后台 - 角色创建/更新 Request VO
#[derive(Debug, Deserialize, Validate)]
pub struct RoleSaveReqVo {
    /// 角色标志
    pub code: String,
    /// 角色编号
    pub id: Option<String>,
    /// 角色名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态
    pub status: CommonStatusEnum,
}

/// TenantSaveReqVO，管理后台 - 租户创建 Request VO
#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantSaveReqVo {
    /// 账号数量
    #[validate(range(min = 0, message = "账号数量不能小于0"))]
    pub account_count: i32,
    /// 联系手机
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub contact_mobile: Option<String>,
    /// 联系人
    pub contact_name: String,
    /// 过期时间
    #[serde(with = "datetime_format")]
    pub expire_time: DateTime,
    /// 租户名
    pub name: String,
    /// 租户套餐编号
    pub package_id: String,
    /// 密码
    pub password: String,
    /// 租户状态
    pub status: CommonStatusEnum,
    /// 用户账号
    #[validate(custom(function = "validation::is_valid_username"))]
    pub username: String,
    /// 绑定域名数组
    pub websites: Option<Vec<String>>,
}

/// TenantUpdateReqVo，管理后台 - 租户修改 Request VO
#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantUpdateReqVo {
    /// 账号数量
    #[validate(range(min = 0, message = "账号数量不能小于0"))]
    pub account_count: i32,
    /// 联系手机
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub contact_mobile: Option<String>,
    /// 联系人
    pub contact_name: String,
    /// 过期时间
    #[serde(with = "datetime_format")]
    pub expire_time: DateTime,
    /// 租户编号
    pub id: String,
    /// 租户名
    pub name: String,
    /// 租户套餐编号
    pub package_id: String,
    /// 租户状态
    pub status: CommonStatusEnum,
    /// 绑定域名数组
    pub websites: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthLoginReqVO {
    #[validate(
        length(min = 4, max = 16, message = "账号长度为4-16"),
        custom(function = "validation::is_valid_username")
    )]
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
    pub websites: Option<Vec<String>>,
    pub package_id: String,
    #[serde(with = "datetime_format")]
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
