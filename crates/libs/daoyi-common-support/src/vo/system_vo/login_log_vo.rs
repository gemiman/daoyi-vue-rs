use crate::vo::system_vo::option_vec_datetime_format;
use sea_orm::prelude::DateTime;
use crate::enumeration::{CommonStatusEnum, LoginLogTypeEnum, LoginResultEnum, UserTypeEnum};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::pagination::PaginationParams;

/// LoginLogRespVO，管理后台 - 登录日志 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginLogRespVO {
    /// 登录时间
    pub create_time: String,
    /// 日志编号
    pub id: String,
    /// 日志类型，参见 LoginLogTypeEnum 枚举类
    pub log_type: LoginLogTypeEnum,
    /// 登录结果，参见 LoginResultEnum 枚举类
    pub result: LoginResultEnum,
    /// 链路追踪编号
    pub trace_id: Option<String>,
    /// 浏览器 UserAgent
    pub user_agent: Option<String>,
    /// 用户编号
    pub user_id: Option<String>,
    /// 用户 IP
    pub user_ip: String,
    /// 用户账号
    pub username: String,
    /// 用户类型，参见 UserTypeEnum 枚举
    pub user_type: UserTypeEnum,
}

/// 管理后台 - 登录日志分页列表 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginLogPageReqVO {
    /// 用户 IP，模拟匹配
    pub user_ip: Option<String>,
    /// 用户账号，模拟匹配
    pub username: Option<String>,
    /// 操作状态
    pub status: Option<bool>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}