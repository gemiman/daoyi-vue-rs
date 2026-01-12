use crate::enumeration::{CommonStatusEnum, DataScopeEnum, MenuTypeEnum, RoleTypeEnum, SexEnum};
use crate::models::FlexibleInt;
use crate::models::pagination::PaginationParams;
use crate::request::validation;
use crate::serde::datetime_format;
use crate::serde::de_comma_separated;
use crate::serde::option_datetime_format;
use crate::serde::option_vec_datetime_format;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// PermissionAssignUserRoleReqVO，管理后台 - 赋予用户角色 Request VO
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignUserRoleReqVO {
    /// 角色编号列表
    pub role_ids: Option<Vec<String>>,
    /// 用户编号
    pub user_id: String,
}

/// PermissionAssignRoleMenuReqVO，管理后台 - 赋予角色菜单 Request VO
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignRoleMenuReqVO {
    /// 菜单编号列表
    pub menu_ids: Option<Vec<String>>,
    /// 角色编号
    pub role_id: String,
}

/// PermissionAssignRoleDataScopeReqVO，管理后台 - 赋予角色数据权限 Request VO
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignRoleDataScopeReqVO {
    /// 数据范围，参见 DataScopeEnum 枚举类
    pub data_scope: DataScopeEnum,
    /// 部门编号列表，只有范围类型为 DEPT_CUSTOM 时，该字段才需要
    pub data_scope_dept_ids: Option<Vec<String>>,
    /// 角色编号
    pub role_id: String,
}

/// UserSaveReqVO，管理后台 - 用户创建/修改 Request VO
#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserSaveReqVO {
    /// 用户账号
    #[validate(
        length(min = 4, max = 16, message = "账号长度为4-16"),
        custom(function = "validation::is_valid_username")
    )]
    pub username: String,
    /// 用户昵称
    #[validate(length(max = 30, message = "用户昵称长度不能超过30个字符"))]
    pub nickname: String,
    /// 备注
    pub remark: Option<String>,
    /// 部门编号
    pub dept_id: Option<String>,
    /// 岗位编号数组
    pub post_ids: Option<Vec<String>>,
    /// 用户邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    /// 手机号码
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub mobile: Option<String>,
    /// 用户性别，参见 SexEnum 枚举类
    pub sex: Option<SexEnum>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 密码
    #[validate(length(min = 4, max = 16, message = "密码长度为4-16"))]
    pub password: String,
}
/// UserUpdatePasswordReqVO，管理后台 - 用户更新密码 Request VO
#[derive(Debug, Validate, Deserialize)]
pub struct UserUpdatePasswordReqVo {
    /// 用户编号
    pub id: String,
    /// 密码
    #[validate(length(min = 4, max = 16, message = "密码长度为4-16"))]
    pub password: String,
}
#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateReqVO {
    /// 用户编号
    pub id: String,
    /// 用户账号
    #[validate(
        length(min = 4, max = 16, message = "账号长度为4-16"),
        custom(function = "validation::is_valid_username")
    )]
    pub username: String,
    /// 用户昵称
    #[validate(length(max = 30, message = "用户昵称长度不能超过30个字符"))]
    pub nickname: String,
    /// 备注
    pub remark: Option<String>,
    /// 部门编号
    pub dept_id: Option<String>,
    /// 岗位编号数组
    pub post_ids: Option<Vec<String>>,
    /// 用户邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    /// 手机号码
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub mobile: Option<String>,
    /// 用户性别，参见 SexEnum 枚举类
    pub sex: Option<SexEnum>,
    /// 用户头像
    pub avatar: Option<String>,
}

impl From<&TenantSaveReqVo> for UserSaveReqVO {
    fn from(value: &TenantSaveReqVo) -> Self {
        Self {
            username: value.username.clone(),
            password: value.password.clone(),
            nickname: value.contact_name.clone(),
            mobile: value.contact_mobile.clone(),
            ..Default::default()
        }
    }
}

impl From<TenantSaveReqVo> for UserSaveReqVO {
    fn from(value: TenantSaveReqVo) -> Self {
        Self {
            username: value.username,
            password: value.password,
            nickname: value.contact_name,
            mobile: value.contact_mobile,
            ..Default::default()
        }
    }
}

/// TenantSaveReqVO，管理后台 - 租户创建 Request VO
#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantSaveReqVo {
    /// 账号数量
    #[validate(range(min = 0, message = "账号数量不能小于0"))]
    pub account_count: i32,
    /// 联系手机
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub contact_mobile: Option<String>,
    /// 联系人
    pub contact_name: String,
    /// 过期时间
    #[serde(with = "datetime_format")]
    pub expire_time: DateTime,
    /// 租户名
    pub name: String,
    /// 租户套餐编号
    pub package_id: String,
    /// 密码
    pub password: String,
    /// 租户状态
    pub status: CommonStatusEnum,
    /// 用户账号
    #[validate(custom(function = "validation::is_valid_username"))]
    pub username: String,
    /// 绑定域名数组
    pub websites: Option<Vec<String>>,
}

/// TenantUpdateReqVo，管理后台 - 租户修改 Request VO
#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantUpdateReqVo {
    /// 账号数量
    #[validate(range(min = 0, message = "账号数量不能小于0"))]
    pub account_count: i32,
    /// 联系手机
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub contact_mobile: Option<String>,
    /// 联系人
    pub contact_name: String,
    /// 过期时间
    #[serde(with = "datetime_format")]
    pub expire_time: DateTime,
    /// 租户编号
    pub id: String,
    /// 租户名
    pub name: String,
    /// 租户套餐编号
    pub package_id: String,
    /// 租户状态
    pub status: CommonStatusEnum,
    /// 绑定域名数组
    pub websites: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthLoginReqVO {
    #[validate(
        length(min = 4, max = 16, message = "账号长度为4-16"),
        custom(function = "validation::is_valid_username")
    )]
    pub username: String,
    #[validate(length(min = 4, max = 16, message = "密码长度为4-16"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginRespVO {
    pub tenant_id: String,
    pub user_id: String,
    pub access_token: String,
    #[serde(with = "datetime_format")]
    pub expires_time: DateTime,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantRespVO {
    pub id: String,
    pub name: String,
    pub contact_user_id: Option<String>,
    pub contact_name: String,
    pub contact_mobile: Option<String>,
    pub status: CommonStatusEnum,
    pub websites: Option<Vec<String>>,
    pub package_id: String,
    #[serde(with = "datetime_format")]
    pub expire_time: DateTime,
    pub account_count: i32,
}

/// DictTypeSaveReqVO，管理后台 - 字典类型创建/修改 Request VO
#[derive(Debug, Validate, Deserialize)]
pub struct DictTypeSaveReqVO {
    /// 字典名称
    #[validate(length(max = 100, message = "字典类型名称长度不能超过100个字符"))]
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: CommonStatusEnum,
    /// 字典类型
    #[serde(rename = "type")]
    #[validate(length(max = 100, message = "字典类型类型长度不能超过 100 个字符"))]
    pub r#type: String,
}
#[derive(Debug, Validate, Deserialize)]
pub struct DictTypeUpdateReqVO {
    /// 字典类型编号
    pub id: String,
    /// 字典名称
    #[validate(length(max = 100, message = "字典类型名称长度不能超过100个字符"))]
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: CommonStatusEnum,
    /// 字典类型
    #[serde(rename = "type")]
    #[validate(length(max = 100, message = "字典类型类型长度不能超过 100 个字符"))]
    pub r#type: String,
}

/// DictTypeRespVO，管理后台 - 字典类型信息 Response VO
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeSimpleRespVO {
    /// 字典类型编号
    pub id: String,
    /// 字典名称
    pub name: String,
    /// 字典类型
    #[serde(rename = "type")]
    pub r#type: String,
}

/// DictTypeRespVO，管理后台 - 字典类型信息 Response VO
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeRespVO {
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 字典类型编号
    pub id: String,
    /// 字典名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: CommonStatusEnum,
    /// 字典类型
    #[serde(rename = "type")]
    pub r#type: String,
}
/// 管理后台 - 字典类型分页列表 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DictTypePageReqVO {
    /// 字典类型名称，模糊匹配
    pub name: Option<String>,
    /// 字典类型，模糊匹配
    #[validate(length(max = 100, message = "字典类型类型长度不能超过100个字符"))]
    pub r#type: Option<String>,
    /// 状态（0正常 1停用）展示状态，参见 CommonStatusEnum 枚举类
    pub status: Option<CommonStatusEnum>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DictDataSimpleRespVO {
    pub dict_type: String,
    pub value: String,
    pub label: String,
    pub color_type: Option<String>,
    pub css_class: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthPermissionInfoRespVO {
    pub user: UserVO,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub menus: Vec<MenuVO>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserVO {
    pub id: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub dept_id: Option<String>,
    pub username: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuVO {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub path: Option<String>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub icon: Option<String>,
    pub visible: bool,
    pub keep_alive: bool,
    pub always_show: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MenuVO>,
}

/// MenuSimpleRespVO，管理后台 - 菜单精简信息 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuSimpleRespVo {
    /// 菜单编号
    pub id: String,
    /// 菜单名称
    pub name: String,
    /// 父菜单 ID
    pub parent_id: String,
    /// 类型，参见 MenuTypeEnum 枚举类
    pub r#type: MenuTypeEnum,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuSaveVO {
    #[validate(length(max = 50, message = "菜单名称长度不能超过50个字符"))]
    pub name: String,
    pub permission: Option<String>,
    pub r#type: MenuTypeEnum,
    pub sort: i32,
    pub parent_id: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub status: CommonStatusEnum,
    pub visible: bool,
    pub keep_alive: bool,
    pub always_show: bool,
}

/// DeptSaveReqVO，管理后台 - 部门创建/修改 Request VO
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptSaveReqVO {
    /// 邮箱
    pub email: Option<String>,
    /// 负责人的用户编号
    pub leader_user_id: Option<String>,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_id: Option<String>,
    /// 联系电话
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub phone: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: CommonStatusEnum,
}
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptUpdateReqVO {
    /// 邮箱
    pub email: Option<String>,
    /// 部门编号
    pub id: String,
    /// 负责人的用户编号
    pub leader_user_id: Option<String>,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_id: Option<String>,
    /// 联系电话
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub phone: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: CommonStatusEnum,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuUpdateVO {
    pub id: String,
    #[validate(length(max = 50, message = "菜单名称长度不能超过50个字符"))]
    pub name: String,
    pub permission: Option<String>,
    pub r#type: MenuTypeEnum,
    pub sort: i32,
    pub parent_id: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub status: CommonStatusEnum,
    pub visible: bool,
    pub keep_alive: bool,
    pub always_show: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRespVO {
    pub id: String,
    pub name: String,
    pub permission: Option<String>,
    pub r#type: MenuTypeEnum,
    pub sort: i32,
    pub parent_id: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub status: CommonStatusEnum,
    pub visible: bool,
    pub keep_alive: bool,
    pub always_show: bool,
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
}

/// 管理后台 - 部门精简信息 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptSimpleRespVO {
    /// 部门编号
    pub id: String,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_id: String,
}

/// DeptRespVO，管理后台 - 部门信息 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptRespVo {
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 邮箱
    pub email: Option<String>,
    /// 部门编号
    pub id: String,
    /// 负责人的用户编号
    pub leader_user_id: Option<String>,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_id: String,
    /// 联系电话
    pub phone: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: CommonStatusEnum,
}

/// 管理后台 - 部门列表 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeptListReqVO {
    /// 部门名称，模糊匹配
    pub name: Option<String>,
    /// 展示状态，参见 CommonStatusEnum 枚举类
    pub status: Option<CommonStatusEnum>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuListReqVO {
    pub name: Option<String>,
    pub status: Option<CommonStatusEnum>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NameParams {
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct WebsiteParams {
    pub website: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserIdParams {
    pub user_id: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TenantIdParams {
    pub tenant_id: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TokenParams {
    pub token: String,
}

/// 管理后台 - 角色分页 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserPageReqVO {
    /// 用户账号，模糊匹配
    pub username: Option<String>,
    /// 手机号码，模糊匹配
    pub mobile: Option<String>,
    /// 状态（0正常 1停用）展示状态，参见 CommonStatusEnum 枚举类
    pub status: Option<CommonStatusEnum>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    /// 部门编号，同时筛选子部门
    pub dept_id: Option<String>,
    /// 角色编号
    pub role_id: Option<String>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}

/// UserRespVO，管理后台 - 用户信息 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRespVO {
    /// 用户头像
    pub avatar: Option<String>,
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 部门ID
    pub dept_id: Option<String>,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户编号
    pub id: String,
    /// 最后登录时间
    #[serde(with = "option_datetime_format")]
    pub login_date: Option<DateTime>,
    /// 最后登录 IP
    pub login_ip: Option<String>,
    /// 手机号码
    pub mobile: Option<String>,
    /// 用户昵称
    pub nickname: String,
    /// 岗位编号数组
    pub post_ids: Option<Vec<String>>,
    /// 备注
    pub remark: Option<String>,
    /// 用户性别，参见 SexEnum 枚举类
    pub sex: Option<SexEnum>,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: CommonStatusEnum,
    /// 用户账号
    pub username: String,
}

/// 管理后台 - 角色分页 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RolePageReqVO {
    /// 角色名称，模糊匹配
    pub name: Option<String>,
    /// 状态（0正常 1停用）展示状态，参见 CommonStatusEnum 枚举类
    pub status: Option<CommonStatusEnum>,
    /// 角色标识，模糊匹配
    pub code: Option<String>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}

/// RoleRespVO，管理后台 - 角色信息 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRespVO {
    /// 角色标志
    pub code: String,
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 数据范围，参见 DataScopeEnum 枚举类
    pub data_scope: DataScopeEnum,
    /// 数据范围(指定部门数组)
    pub data_scope_dept_ids: Vec<String>,
    /// 角色编号
    pub id: String,
    /// 角色名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: CommonStatusEnum,
    /// 角色类型，参见 RoleTypeEnum 枚举类
    pub r#type: RoleTypeEnum,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TenantPackagePageReqVO {
    /// 套餐名
    pub name: Option<String>,
    /// 状态（0正常 1停用）
    pub status: Option<CommonStatusEnum>,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TenantPageReqVo {
    /// 联系手机
    pub contact_mobile: Option<String>,
    /// 联系人
    pub contact_name: Option<String>,
    /// 创建时间
    #[serde(default)]
    #[serde(with = "option_vec_datetime_format")]
    pub create_time: Option<Vec<DateTime>>,
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

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdParams {
    pub id: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoleIdParams {
    pub role_id: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdsParams {
    #[serde(deserialize_with = "de_comma_separated")]
    #[validate(length(min = 1, message = "ids不能为空"))]
    pub ids: Vec<String>,
}

/// TenantPackageSaveReqVO，管理后台 - 租户套餐创建/修改 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TenantPackageSaveReqVo {
    /// 关联的菜单编号
    pub menu_ids: Vec<String>,
    /// 套餐名
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 状态，参见 CommonStatusEnum 枚举
    pub status: CommonStatusEnum,
}
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TenantPackageUpdateReqVo {
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

/// UserSimpleRespVO，管理后台 - 用户精简信息 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSimpleRespVo {
    /// 部门ID
    pub dept_id: Option<String>,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 用户编号
    pub id: String,
    /// 用户昵称
    pub nickname: String,
}

/// 管理后台 - 岗位分页 Request VO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PostPageReqVO {
    /// 岗位名称，模糊匹配
    pub name: Option<String>,
    /// 状态（0正常 1停用）
    pub status: Option<CommonStatusEnum>,
    /// 岗位编码，模糊匹配
    pub code: Option<String>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PaginationParams,
}
/// PostRespVO，管理后台 - 岗位信息 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRespVo {
    /// 岗位编码
    pub code: String,
    /// 创建时间
    #[serde(with = "datetime_format")]
    pub create_time: DateTime,
    /// 岗位序号
    pub id: String,
    /// 岗位名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: CommonStatusEnum,
}
/// PostSaveReqVO，管理后台 - 岗位创建/修改 Request VO
#[derive(Debug, Validate, Deserialize)]
pub struct PostSaveReqVO {
    /// 岗位编码
    #[validate(length(max = 64, message = "岗位名称长度不能超过 64 个字符"))]
    pub code: String,
    /// 岗位名称
    #[validate(length(max = 50, message = "岗位名称长度不能超过 50 个字符"))]
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: FlexibleInt<i32>,
    /// 状态
    pub status: CommonStatusEnum,
}
#[derive(Debug, Validate, Deserialize)]
pub struct PostUpdateReqVo {
    /// 岗位编码
    #[validate(length(max = 64, message = "岗位名称长度不能超过 64 个字符"))]
    pub code: String,
    /// 岗位编号
    pub id: String,
    /// 岗位名称
    #[validate(length(max = 50, message = "岗位名称长度不能超过 50 个字符"))]
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: FlexibleInt<i32>,
    /// 状态
    pub status: CommonStatusEnum,
}
/// PostSimpleRespVO，管理后台 - 岗位信息的精简 Response VO
#[derive(Debug, Serialize)]
pub struct PostSimpleRespVo {
    /// 岗位序号
    pub id: String,
    /// 岗位名称
    pub name: String,
}

/// RoleSaveReqVO，管理后台 - 角色创建/更新 Request VO
#[derive(Debug, Deserialize, Validate)]
pub struct RoleSaveReqVO {
    /// 角色标志
    #[validate(length(max = 100, message = "角色标志长度不能超过 100 个字符"))]
    pub code: String,
    /// 角色名称
    #[validate(length(max = 30, message = "角色名称长度不能超过 30 个字符"))]
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: FlexibleInt<i32>,
    /// 状态
    pub status: CommonStatusEnum,
}
#[derive(Debug, Deserialize, Validate)]
pub struct RoleUpdateReqVO {
    /// 角色标志
    #[validate(length(max = 100, message = "角色标志长度不能超过 100 个字符"))]
    pub code: String,
    /// 角色编号
    pub id: String,
    /// 角色名称
    #[validate(length(max = 30, message = "角色名称长度不能超过 30 个字符"))]
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: FlexibleInt<i32>,
    /// 状态
    pub status: CommonStatusEnum,
}
