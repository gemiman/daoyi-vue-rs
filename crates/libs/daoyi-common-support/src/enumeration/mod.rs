pub mod redis_keys;

use daoyi_macros::DaoyiIntoActiveValue;
use sea_orm::prelude::*;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

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
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CommonStatusEnum {
    #[sea_orm(string_value = "0")]
    #[serde(rename = "0")]
    Enable,
    #[sea_orm(string_value = "1")]
    #[serde(rename = "1")]
    Disable,
}

impl<'de> Deserialize<'de> for CommonStatusEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommonStatusVisitor;

        impl<'de> Visitor<'de> for CommonStatusVisitor {
            type Value = CommonStatusEnum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("integer 0/1 or string '0'/'1'")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    0 => Ok(CommonStatusEnum::Enable),
                    1 => Ok(CommonStatusEnum::Disable),
                    _ => Err(E::custom(format!("Invalid status value: {}", v))),
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    0 => Ok(CommonStatusEnum::Enable),
                    1 => Ok(CommonStatusEnum::Disable),
                    _ => Err(E::custom(format!("Invalid status value: {}", v))),
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "0" => Ok(CommonStatusEnum::Enable),
                    "1" => Ok(CommonStatusEnum::Disable),
                    _ => Err(E::custom(format!("Invalid status value: {}", v))),
                }
            }
        }

        deserializer.deserialize_any(CommonStatusVisitor)
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
