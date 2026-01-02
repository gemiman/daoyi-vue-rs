use crate::enumeration::FileStorageEnum;
use crate::models::pagination::PaginationParams;
use crate::serde::datetime_format;
use crate::serde::option_vec_datetime_format;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// FileConfigRespVO，管理后台 - 文件配置 Response VO
#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FileConfigPageReqVO {
    /// 存储器
    pub storage: Option<FileStorageEnum>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    /// 配置名
    pub name: Option<String>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}
/// FileConfigSaveReqVO，管理后台 - 文件配置创建/修改 Request VO
#[derive(Debug, Deserialize, Validate)]
pub struct FileConfigSaveReqVo {
    /// 存储配置,配置是动态参数，所以使用 Json 接收
    pub config: Json,
    /// 配置名
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 存储器，参见 FileStorageEnum 枚举类
    pub storage: FileStorageEnum,
}
