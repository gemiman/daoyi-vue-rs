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
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum LoginResultEnum {
    //     SUCCESS(0), // 成功
    #[sea_orm(string_value = "0")]
    Success,
    //     BAD_CREDENTIALS(10), // 账号或密码不正确
    #[sea_orm(string_value = "10")]
    BadCredentials,
    //     USER_DISABLED(20), // 用户被禁用
    #[sea_orm(string_value = "20")]
    UserDisabled,
    //     CAPTCHA_NOT_FOUND(30), // 图片验证码不存在
    #[sea_orm(string_value = "30")]
    CaptchaNotFound,
    //     CAPTCHA_CODE_ERROR(31), // 图片验证码不正确
    #[sea_orm(string_value = "31")]
    CaptchaCodeError,
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
pub enum LoginLogTypeEnum {
    //    LOGIN_USERNAME(100), // 使用账号登录
    #[sea_orm(string_value = "100")]
    LoginUsername,
    //     LOGIN_SOCIAL(101), // 使用社交登录
    #[sea_orm(string_value = "101")]
    LoginSocial,
    //     LOGIN_MOBILE(103), // 使用手机登陆
    #[sea_orm(string_value = "103")]
    LoginMobile,
    //     LOGIN_SMS(104), // 使用短信登陆
    #[sea_orm(string_value = "104")]
    LoginSms,
    //     LOGOUT_SELF(200),  // 自己主动登出
    #[sea_orm(string_value = "200")]
    LogoutSelf,
    //     LOGOUT_DELETE(202), // 强制退出
    #[sea_orm(string_value = "202")]
    LogoutDelete,
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

/// 企业状态 状态(0:禁用企业,1:免费企业;2:试用企业,3:付费企业)
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
pub enum CompanyStatusEnum {
    /// 禁用企业
    #[sea_orm(string_value = "0")]
    Disable,
    /// 免费企业
    #[sea_orm(string_value = "1")]
    Free,
    /// 试用企业
    #[sea_orm(string_value = "2")]
    Trial,
    /// 付费企业
    #[sea_orm(string_value = "3")]
    Payed,
}

/// 计费方式 1:呼出计费,2:呼入计费,3:双向计费,0:全免费
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
pub enum BillingTypeEnum {
    /// 全免费
    #[sea_orm(string_value = "0")]
    Free,
    /// 呼出计费
    #[sea_orm(string_value = "1")]
    BillingOut,
    /// 呼入计费
    #[sea_orm(string_value = "2")]
    BillingIn,
    /// 双向计费
    #[sea_orm(string_value = "3")]
    BillingBoth,
}

/// 支付方式 0:预付费;1:后付费
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
pub enum PayTypeEnum {
    /// 预付费
    #[sea_orm(string_value = "0")]
    Prepaid,
    /// 后付费
    #[sea_orm(string_value = "1")]
    Postpaid,
}

/// 坐席密码等级(1:不限制 2:数字和字母 3:大小写字母和数字组合)
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
pub enum AgentPasswordTypeEnum {
    /// 不限制
    #[sea_orm(string_value = "1")]
    NoRestriction,
    /// 数字和字母
    #[sea_orm(string_value = "2")]
    NumberAndLetter,
    /// 大小写字母和数字组合
    #[sea_orm(string_value = "3")]
    UpperLowerAndNumber,
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
    strum_macros::Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SmsChannelEnum {
    #[sea_orm(string_value = "0")]
    DebugDingTalk,
    #[sea_orm(string_value = "1")]
    Aliyun,
    #[sea_orm(string_value = "2")]
    Tencent,
    #[sea_orm(string_value = "3")]
    Huawei,
    #[sea_orm(string_value = "4")]
    Qiniu,
}

/// 短信模板的审核状态枚举
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
pub enum SmsTemplateAuditStatusEnum {
    #[sea_orm(string_value = "1")]
    CHECKING,
    #[sea_orm(string_value = "2")]
    SUCCESS,
    #[sea_orm(string_value = "3")]
    FAIL,
}

/// 短信的模板类型枚举
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
    strum_macros::Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SmsTemplateTypeEnum {
    #[sea_orm(string_value = "1")]
    VerificationCode,
    #[sea_orm(string_value = "2")]
    Notice,
    #[sea_orm(string_value = "3")]
    Promotion,
}

/// 代码生成的场景枚举
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
    strum_macros::Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CodegenSceneEnum {
    /// 管理后台
    #[sea_orm(string_value = "1")]
    ADMIN,
    /// 用户 APP
    #[sea_orm(string_value = "2")]
    APP,
}

/// 代码生成模板类型
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
    strum_macros::Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CodegenTemplateTypeEnum {
    /// 单表（增删改查）
    #[sea_orm(string_value = "1")]
    ONE,
    /// 树表（增删改查）
    #[sea_orm(string_value = "2")]
    TREE,
    ///     MasterNormal(10), // 主子表 - 主表 - 普通模式
    #[sea_orm(string_value = "10")]
    MasterNormal,
    //     MasterErp(11), // 主子表 - 主表 - ERP 模式
    #[sea_orm(string_value = "11")]
    MasterErp,
    //     MasterInner(12), // 主子表 - 主表 - 内嵌模式
    #[sea_orm(string_value = "12")]
    MasterInner,
    //     SUB(15), // 主子表 - 子表
    #[sea_orm(string_value = "15")]
    SUB,
}

/// 代码生成模板类型
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
    strum_macros::Display,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CodegenFrontTypeEnum {
    ///     VUE2_ELEMENT_UI(10), // Vue2 Element UI 标准模版
    #[sea_orm(string_value = "10")]
    Vue2ElementUi,
    //     VUE3_ELEMENT_PLUS(20), // Vue3 Element Plus 标准模版
    #[sea_orm(string_value = "20")]
    Vue3ElementPlus,
    //     VUE3_VBEN2_ANTD_SCHEMA(30), // Vue3 VBEN2 + ANTD + Schema 模版
    #[sea_orm(string_value = "30")]
    Vue3Vben2AntdSchema,
    //     VUE3_VBEN5_ANTD_SCHEMA(40), // Vue3 VBEN5 + ANTD + schema 模版
    #[sea_orm(string_value = "40")]
    Vue3Vben5AntdSchema,
    //     VUE3_VBEN5_ANTD_GENERAL(41), // Vue3 VBEN5 + ANTD 标准模版
    #[sea_orm(string_value = "41")]
    Vue3Vben5AntdGeneral,
    //     VUE3_VBEN5_EP_SCHEMA(50), // Vue3 VBEN5 + EP + schema 模版
    #[sea_orm(string_value = "50")]
    Vue3Vben5EpSchema,
    //     VUE3_VBEN5_EP_GENERAL(51), // Vue3 VBEN5 + EP 标准模版
    #[sea_orm(string_value = "51")]
    Vue3Vben5EpGeneral,
    //     VUE3_ADMIN_UNIAPP_WOT(60), // Vue3 Admin + Uniapp + WOT 标准模版
    #[sea_orm(string_value = "60")]
    Vue3AdminUniappWot,
}

/// 区域类型枚举
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumIter,
    DeriveActiveEnum,
    DaoyiIntoActiveValue,
    DaoyiStringOrNumberSerde,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AreaTypeEnum {
    ///     全球,
    #[sea_orm(string_value = "0")]
    EARTH,
    ///     COUNTRY(1, "国家"),
    #[sea_orm(string_value = "1")]
    COUNTRY,
    ///     PROVINCE(2, "省份"),
    #[sea_orm(string_value = "2")]
    PROVINCE,
    ///     CITY(3, "城市"),
    #[sea_orm(string_value = "3")]
    CITY,
    ///     DISTRICT(4, "地区"), // 县、镇、区等
    #[sea_orm(string_value = "4")]
    DISTRICT,
}
