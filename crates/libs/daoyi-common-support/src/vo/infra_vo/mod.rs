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
#[derive(Debug, Deserialize, Validate)]
pub struct FileConfigUpdateReqVo {
    pub id: String,
    /// 存储配置,配置是动态参数，所以使用 Json 接收
    pub config: Json,
    /// 配置名
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 存储器，参见 FileStorageEnum 枚举类
    pub storage: FileStorageEnum,
}


#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DbFileClientConfig {
    #[validate(url(message = "domain 必须是 URL 格式"), length(min = 1, message = "domain 不能为空"))]
    pub domain: String,
}

#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalFileClientConfig {
    #[validate(length(min = 1, message = "基础路径不能为空"))]
    pub base_path: String,
    #[validate(url(message = "domain 必须是 URL 格式"), length(min = 1, message = "domain 不能为空"))]
    pub domain: String,
}

#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FtpFileClientConfig {
    #[validate(length(min = 1, message = "基础路径不能为空"))]
    pub base_path: String,
    #[validate(url(message = "domain 必须是 URL 格式"), length(min = 1, message = "domain 不能为空"))]
    pub domain: String,
    #[validate(length(min = 1, message = "host 不能为空"))]
    pub host: String,
    pub port: u16,
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
    #[validate(length(min = 1, message = "连接模式不能为空"))]
    pub mode: String,
}

#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SftpFileClientConfig {
    #[validate(length(min = 1, message = "基础路径不能为空"))]
    pub base_path: String,
    #[validate(url(message = "domain 必须是 URL 格式"), length(min = 1, message = "domain 不能为空"))]
    pub domain: String,
    #[validate(length(min = 1, message = "host 不能为空"))]
    pub host: String,
    pub port: u16,
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct S3FileClientConfig {
    #[validate(length(min = 1, message = "endpoint 不能为空"))]
    pub endpoint: String,
    #[validate(url(message = "domain 必须是 URL 格式"))]
    pub domain: Option<String>,
    #[validate(length(min = 1, message = "bucket 不能为空"))]
    pub bucket: String,
    #[validate(length(min = 1, message = "accessKey 不能为空"))]
    pub access_key: String,
    #[validate(length(min = 1, message = "accessSecret 不能为空"))]
    pub access_secret: String,
    #[allow(dead_code)]
    pub enable_path_style_access: bool,
    #[allow(dead_code)]
    pub enable_public_access: bool,
    #[allow(dead_code)]
        pub region: Option<String>,
    }
    
    // ================== File VO ==================
    
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FileRespVO {
        pub id: String,
        pub config_id: Option<String>,
        pub name: Option<String>,
        pub path: String,
        pub url: String,
        pub r#type: Option<String>,
        pub size: i32,
        #[serde(with = "datetime_format")]
        pub create_time: DateTime,
    }
    
    #[derive(Debug, Deserialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct FilePageReqVO {
        pub path: Option<String>,
        pub r#type: Option<String>,
        #[serde(default)]
        #[serde(with = "option_vec_datetime_format")]
        pub create_time: Option<Vec<DateTime>>,
        #[serde(flatten)]
        #[validate(nested)]
        pub pagination: PaginationParams,
    }
    
    #[derive(Debug, Deserialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct FileCreateReqVO {
        pub config_id: Option<String>,
        #[validate(length(min = 1, message = "路径不能为空"))]
        pub path: String,
        #[validate(length(min = 1, message = "原文件名不能为空"))]
        pub name: String,
        pub r#type: Option<String>,
        pub size: i32,
        #[validate(url(message = "url 必须是 URL 格式"))]
        pub url: String,
    }
    
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilePresignedUrlRespVO {
        pub upload_url: String,
        pub url: String,
    }
    