use crate::enumeration::CommonStatusEnum;
use crate::models::pagination::PaginationParams;
use serde::Deserialize;
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
