pub mod redis_keys;

use daoyi_macros::{DaoyiIntoActiveValue, DaoyiStringOrNumberSerde};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 套餐编号 - 系统
/// 菜单编号 - 根节点
/// 部门编号 - 根节点
pub const ID_ROOT: &str = "0";
pub const WEBSOCKET_REDIS_CHANNEL: &str = "daoyi.websocket.redis.channel";

pub const ADMIN_API: &str = "/admin-api";
pub const APP_API: &str = "/app-api";

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Gender {
    // #[sea_orm(string_value = "unknown")]
    // Unknown,
    // #[sea_orm(string_value = "male")]
    Male,
    // #[sea_orm(string_value = "female")]
    Female,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SexEnum {
    #[sea_orm(string_value = "1")]
    MALE,
    #[sea_orm(string_value = "2")]
    FEMALE,
    #[sea_orm(string_value = "0")]
    UNKNOWN,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CommonStatusEnum {
    #[sea_orm(string_value = "0")]
    Enable,
    #[sea_orm(string_value = "1")]
    Disable,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum NotifyTemplateTypeEnum {
    #[sea_orm(string_value = "1")]
    NotificationMessage,
    #[sea_orm(string_value = "2")]
    SystemMessage,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum UserTypeEnum {
    #[sea_orm(string_value = "1")]
    Member,
    #[sea_orm(string_value = "2")]
    Admin,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum NoticeTypeEnum {
    #[sea_orm(string_value = "1")]
    NOTICE,
    #[sea_orm(string_value = "2")]
    ANNOUNCEMENT,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum RoleTypeEnum {
    #[sea_orm(string_value = "1")]
    SYSTEM,
    #[sea_orm(string_value = "2")]
    CUSTOM,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum DataScopeEnum {
    #[sea_orm(string_value = "1")]
    ALL, // 全部数据权限
    #[sea_orm(string_value = "2")]
    DeptCustom, // 指定部门数据权限
    #[sea_orm(string_value = "3")]
    DeptOnly, // 部门数据权限
    #[sea_orm(string_value = "4")]
    DeptAndChild, // 部门及以下数据权限
    #[sea_orm(string_value = "5")]
    SELF, // 仅本人数据权限
}

/// 角色标识枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoleCodeEnum {
    /// 超级管理员
    SuperAdmin,
    /// 租户管理员
    TenantAdmin,
    /// CRM 管理员 (CRM 系统专用)
    CrmAdmin,
}

impl RoleCodeEnum {
    /// 获取角色编码
    pub fn code(&self) -> &'static str {
        match self {
            RoleCodeEnum::SuperAdmin => "super_admin",
            RoleCodeEnum::TenantAdmin => "tenant_admin",
            RoleCodeEnum::CrmAdmin => "crm_admin",
        }
    }

    /// 获取角色名称
    pub fn name(&self) -> &'static str {
        match self {
            RoleCodeEnum::SuperAdmin => "超级管理员",
            RoleCodeEnum::TenantAdmin => "租户管理员",
            RoleCodeEnum::CrmAdmin => "CRM 管理员",
        }
    }

    /// 根据code字符串获取枚举
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "super_admin" => Some(RoleCodeEnum::SuperAdmin),
            "tenant_admin" => Some(RoleCodeEnum::TenantAdmin),
            "crm_admin" => Some(RoleCodeEnum::CrmAdmin),
            _ => None,
        }
    }

    /// 判断是否为超级管理员
    pub fn is_super_admin(code: &str) -> bool {
        code == Self::SuperAdmin.code()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MenuTypeEnum {
    #[sea_orm(string_value = "1")]
    DIR, // 目录
    #[sea_orm(string_value = "2")]
    MENU, // 菜单
    #[sea_orm(string_value = "3")]
    BUTTON, // 按钮
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum FileStorageEnum {
    #[sea_orm(string_value = "1")]
    DB,
    #[sea_orm(string_value = "10")]
    LOCAL,
    #[sea_orm(string_value = "11")]
    FTP,
    #[sea_orm(string_value = "12")]
    SFTP,
    #[sea_orm(string_value = "20")]
    S3,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MailSendStatusEnum {
    #[sea_orm(string_value = "0")]
    INIT,
    #[sea_orm(string_value = "10")]
    SUCCESS,
    #[sea_orm(string_value = "20")]
    FAILURE,
    #[sea_orm(string_value = "30")]
    IGNORE,
}
