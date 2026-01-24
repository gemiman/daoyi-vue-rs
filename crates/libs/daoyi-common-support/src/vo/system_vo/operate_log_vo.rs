use crate::enumeration::UserTypeEnum;
use crate::models::pagination::PaginationParams;
use crate::serde::datetime_format;
use crate::serde::option_vec_datetime_format;
use sea_orm::prelude::{DateTime, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// OperateLogRespVO，管理后台 - 操作日志 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperateLogRespVO {
    /// 操作明细
    pub action: String,
    /// 操作模块业务编号
    pub biz_id: String,
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 拓展字段
    pub extra: Json,
    /// 日志编号
    pub id: String,
    /// 请求方法名
    pub request_method: Option<String>,
    /// 请求地址
    pub request_url: Option<String>,
    /// 操作名
    pub sub_type: String,
    /// 链路追踪编号
    pub trace_id: String,
    /// 操作模块类型
    pub r#type: String,
    /// 浏览器 UserAgent
    pub user_agent: Option<String>,
    /// 用户编号
    pub user_id: String,
    /// 用户 IP
    pub user_ip: Option<String>,
    /// 用户昵称
    pub user_name: String,
    /// 用户类型
    pub user_type: UserTypeEnum,
}

/// 管理后台 - 操作日志分页列表 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperateLogPageReqVO {
    /// 用户编号
    pub user_id: Option<String>,
    /// 操作模块业务编号
    pub biz_id: Option<String>,
    /// 操作模块，模拟匹配
    pub r#type: Option<String>,
    /// 操作名，模拟匹配
    pub sub_type: Option<String>,
    /// 操作明细，模拟匹配
    pub action: Option<String>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OperateLogCreateReqDTO {
    /// 链路追踪编号
    /// 一般来说，通过链路追踪编号，可以将访问日志，错误日志，链路追踪日志，logger 打印日志等，结合在一起，从而进行排错。
    pub trace_id: String,
    /// 用户编号
    /// 关联 MemberUserDO 的 id 属性，或者 AdminUserDO 的 id 属性
    pub user_id: String,
    /// 用户类型
    /// 关联 UserTypeEnum
    pub user_type: UserTypeEnum,
    /// 操作模块类型
    pub r#type: String,
    /// 操作名
    pub sub_type: String,
    /// 操作模块业务编号
    pub biz_id: String,
    /// 操作内容，记录整个操作的明细
    /// 例如说，修改编号为 1 的用户信息，将性别从男改成女，将姓名从芋道改成源码。
    pub action: String,
    /// 拓展字段，有些复杂的业务，需要记录一些字段 ( JSON 格式 )
    /// 例如说，记录订单编号，{ orderId: "1"}
    pub extra: Json,
    /// 请求方法名
    pub request_method: Option<String>,
    /// 请求地址
    pub request_url: String,
    /// 用户 IP
    pub user_ip: Option<String>,
    /// 浏览器 UA
    pub user_agent: Option<String>,
}
