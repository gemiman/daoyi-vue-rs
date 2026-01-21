use crate::enumeration::{
    CodegenFrontTypeEnum, CodegenSceneEnum, CodegenTemplateTypeEnum, FileStorageEnum,
};
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
    #[validate(
        url(message = "domain 必须是 URL 格式"),
        length(min = 1, message = "domain 不能为空")
    )]
    pub domain: String,
}

#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalFileClientConfig {
    #[validate(length(min = 1, message = "基础路径不能为空"))]
    pub base_path: String,
    #[validate(
        url(message = "domain 必须是 URL 格式"),
        length(min = 1, message = "domain 不能为空")
    )]
    pub domain: String,
}

#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FtpFileClientConfig {
    #[validate(length(min = 1, message = "基础路径不能为空"))]
    pub base_path: String,
    #[validate(
        url(message = "domain 必须是 URL 格式"),
        length(min = 1, message = "domain 不能为空")
    )]
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
    #[validate(
        url(message = "domain 必须是 URL 格式"),
        length(min = 1, message = "domain 不能为空")
    )]
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

#[derive(Debug, Deserialize, Validate)]
pub struct PresignedUrlReq {
    pub name: String,
    pub directory: Option<String>,
}

// ================== DataSourceConfig VO ==================

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceConfigRespVO {
    pub id: String,
    pub name: String,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub schema_name: String,
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceConfigSaveReqVO {
    #[validate(length(min = 1, message = "数据源名称不能为空"))]
    pub name: String,
    #[validate(length(min = 1, message = "数据源连接不能为空"))]
    pub url: String,
    #[validate(length(min = 1, message = "数据库不能为空"))]
    pub schema_name: String,
    pub username: Option<String>,
    pub password: Option<String>,
}
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceConfigUpdateReqVO {
    #[validate(length(min = 1, message = "ID不能为空"))]
    pub id: String,
    #[validate(length(min = 1, message = "数据源名称不能为空"))]
    pub name: String,
    #[validate(length(min = 1, message = "数据源连接不能为空"))]
    pub url: String,
    #[validate(length(min = 1, message = "数据库不能为空"))]
    pub schema_name: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// CodegenTableRespVO，管理后台 - 代码生成表定义 Response VO
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenTableRespVO {
    /// 作者
    pub author: String,
    /// 业务名
    pub business_name: String,
    /// 类描述
    pub class_comment: String,
    /// 类名称
    pub class_name: String,
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 主键编号
    pub data_source_config_id: String,
    /// 前端类型，参见 CodegenFrontTypeEnum 枚举
    pub front_type: CodegenFrontTypeEnum,
    /// 编号
    pub id: String,
    /// 主表的编号
    pub master_table_id: Option<String>,
    /// 模块名
    pub module_name: String,
    /// 父菜单编号
    pub parent_menu_id: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 生成场景，参见 CodegenSceneEnum 枚举
    pub scene: CodegenSceneEnum,
    /// 子表关联主表的字段编号
    pub sub_join_column_id: Option<String>,
    /// 主表与子表是否一对多
    pub sub_join_many: Option<bool>,
    /// 表描述
    pub table_comment: String,
    /// 表名称
    pub table_name: String,
    /// 模板类型，参见 CodegenTemplateTypeEnum 枚举
    pub template_type: CodegenTemplateTypeEnum,
    /// 树表的名字字段编号
    pub tree_name_column_id: Option<String>,
    /// 树表的父字段编号
    pub tree_parent_column_id: Option<String>,
    /// 更新时间
    #[serde(with = "datetime_format")]
    pub update_time: DateTime,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodegenTablePageReqVO {
    pub table_name: Option<String>,
    pub table_comment: Option<String>,
    pub class_name: Option<String>,
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodegenCreateListReqVO {
    pub data_source_config_id: String,
    pub table_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseTableRespVO {
    pub name: String,
    pub comment: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DbTableListReq {
    pub data_source_config_id: String,
    pub name: Option<String>,
    pub comment: Option<String>,
}
/// CodegenColumnRespVO，管理后台 - 代码生成字段定义 Response VO
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenColumnRespVO {
    /// 字段描述
    pub column_comment: String,
    /// 字段名
    pub column_name: String,
    /// 是否为 Create 创建操作的字段
    pub create_operation: bool,
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 字段类型
    pub data_type: String,
    /// 字典类型
    pub dict_type: Option<String>,
    /// 数据示例
    pub example: Option<String>,
    /// 显示类型
    pub html_type: String,
    /// 编号
    pub id: String,
    /// Java 属性名
    pub java_field: String,
    /// Java 属性类型
    pub java_type: String,
    /// 是否为 List 查询操作的字段
    pub list_operation: bool,
    /// List 查询操作的条件类型，参见 CodegenColumnListConditionEnum 枚举
    pub list_operation_condition: String,
    /// 是否为 List 查询操作的返回字段
    pub list_operation_result: bool,
    /// 是否允许为空
    pub nullable: bool,
    /// 排序
    pub ordinal_position: i32,
    /// 是否主键
    pub primary_key: bool,
    /// 表编号
    pub table_id: String,
    /// 是否为 Update 更新操作的字段
    pub update_operation: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenDetailRespVO {
    pub table: Option<CodegenTableRespVO>,
    pub columns: Vec<CodegenColumnRespVO>,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenTableSaveReqVO {
    pub id: String,
    pub table_name: String,
    pub table_comment: String,
    pub class_name: String,
    pub module_name: String,
    pub business_name: String,
    pub scene: CodegenSceneEnum,
    pub template_type: CodegenTemplateTypeEnum,
    pub master_table_id: Option<String>,
    pub sub_join_column_id: Option<String>,
    pub sub_join_many: Option<bool>,
    pub tree_parent_column_id: Option<String>,
    pub tree_name_column_id: Option<String>,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenColumnSaveReqVO {
    pub id: String,
    pub column_name: String,
    pub data_type: String,
    pub column_comment: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub ordinal_position: i32,
    pub java_type: String,
    pub java_field: String,
    pub dict_type: Option<String>,
    pub example: Option<String>,
    pub create_operation: bool,
    pub update_operation: bool,
    pub list_operation: bool,
    pub list_operation_condition: String,
    pub list_operation_result: bool,
    pub html_type: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CodegenUpdateReqVO {
    pub table: CodegenTableSaveReqVO,
    pub columns: Vec<CodegenColumnSaveReqVO>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenPreviewRespVO {
    pub file_path: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TableIdParam {
    pub table_id: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceConfigIdParam {
    pub data_source_config_id: String,
}
