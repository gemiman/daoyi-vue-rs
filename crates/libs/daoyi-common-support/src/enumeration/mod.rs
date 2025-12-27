pub mod redis_keys;

use daoyi_macros::DaoyiIntoActiveValue;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

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
    Serialize,
    Deserialize,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
)]
#[serde(rename_all = "snake_case")]
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
    Serialize,
    Deserialize,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
)]
#[serde(rename_all = "snake_case")]
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
    Serialize,
    Deserialize,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
)]
#[serde(rename_all = "snake_case")]
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
pub enum RoleCodeEnum {
    SuperAdmin,  // 超级管理员
    TenantAdmin, // 租户管理员
    CrmAdmin,    // CRM 管理员
}
impl RoleCodeEnum {
    pub fn is_super_admin(role_code: &str) -> bool {
        role_code == "super_admin"
    }
}

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
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MenuTypeEnum {
    #[sea_orm(string_value = "1")]
    DIR, // 目录
    #[sea_orm(string_value = "2")]
    MENU, // 菜单
    #[sea_orm(string_value = "3")]
    BUTTON, // 按钮
}
