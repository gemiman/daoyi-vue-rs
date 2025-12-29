use crate::enumeration::CommonStatusEnum;
use crate::models::pagination::PaginationParams;
use crate::serde::datetime_format;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TenantPageReqVo {
    /// 联系手机
    pub contact_mobile: Option<String>,
    /// 联系人
    pub contact_name: Option<String>,
    /// 创建时间
    pub create_time: Option<Vec<String>>,
    /// 租户名
    pub name: Option<String>,
    /// 租户状态（0正常 1停用）
    pub status: Option<CommonStatusEnum>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}

/// TenantPackageRespVO，管理后台 - 租户套餐 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantPackageRespVo {
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 套餐编号
    pub id: String,
    /// 关联的菜单编号
    pub menu_ids: Vec<String>,
    /// 套餐名
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 状态，参见 CommonStatusEnum 枚举
    pub status: CommonStatusEnum,
}
