use serde::{Deserialize, Serialize};
use validator::Validate;

/// AreaNodeRespVO，管理后台 - 地区节点 Response VO
#[derive(Debug, Serialize, Deserialize)]
pub struct AreaNodeRespVO {
    /// 编号
    pub id: String,
    /// 名字
    pub name: String,
    /// 子节点
    pub children: Vec<AreaNodeRespVO>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IpParams {
    pub ip: String,
}
