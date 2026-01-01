use crate::enumeration::FileStorageEnum;
use crate::serde::datetime_format;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// FileConfigRespVO，管理后台 - 文件配置 Response VO
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileConfigRespVo {
    /// 存储配置
    pub config: Json,
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 编号
    pub id: String,
    /// 是否为主配置
    pub master: bool,
    /// 配置名
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 存储器，参见 FileStorageEnum 枚举类
    pub storage: FileStorageEnum,
}
