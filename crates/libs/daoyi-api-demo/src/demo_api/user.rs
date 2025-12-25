use anyhow::Context;
use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::Gender;
use daoyi_common_support::error::ApiError;
use daoyi_common_support::models::pagination::{Page, PaginationParams};
use daoyi_common_support::password::hash_password;
use daoyi_common_support::request::path::Path;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::request::validation;
use daoyi_common_support::response::{ApiResponse, ApiResult};
use daoyi_entity_demo::demo_entity::prelude::*;
use daoyi_entity_demo::demo_entity::sys_user;
use daoyi_entity_demo::demo_entity::sys_user::ActiveModel;
use sa_token_plugin_axum::*;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryOrder, QueryTrait};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list", routing::get(query_users))
        .route("/page", routing::get(find_page))
        .route("/", routing::post(create))
        .route("/{id}", routing::put(update))
        .route("/{id}", routing::delete(delete))
}

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    #[validate(length(min = 1, max = 16, message = "姓名长度为1-16"))]
    pub name: String,
    pub gender: Gender,
    #[validate(length(min = 1, max = 16, message = "账号长度为1-16"))]
    pub account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度为6-16"))]
    pub password: String,
    #[validate(custom(function = "validation::is_mobile_phone"))]
    pub mobile_phone: String,
    pub birthday: Date,
    #[serde(default)]
    pub enabled: bool,
}

#[debug_handler]
#[sa_check_permission("user:add")]
async fn create(ValidJson(params): ValidJson<UserParams>) -> ApiResult<sys_user::Model> {
    let active_model = params.into_active_model();
    let model = active_model.insert(database::get().await).await?;
    Ok(ApiResponse::ok(Some(model)))
}

#[debug_handler]
#[sa_check_role("admin")]
async fn update(
    Path(id): Path<String>,
    ValidJson(params): ValidJson<UserParams>,
) -> ApiResult<sys_user::Model> {
    let db = database::get().await;
    let mut existed_active_model = SysUser::find_by_id(&id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待修改的用户不存在")))?
        .into_active_model();
    let old_password = existed_active_model.password.clone();
    let password = params.password.clone();
    let active_model = params.into_active_model();
    existed_active_model.clone_from(&active_model);
    existed_active_model.id = ActiveValue::Unchanged(id);
    if password.is_empty() {
        existed_active_model.password = ActiveValue::Unchanged(old_password.unwrap());
    } else {
        existed_active_model.password = ActiveValue::Set(hash_password(password.as_ref()).await?);
    }
    let model = existed_active_model.update(db).await?;
    Ok(ApiResponse::ok(Some(model)))
}

#[debug_handler]
#[sa_check_permission("user:delete")]
async fn delete(Path(id): Path<String>) -> ApiResult<u64> {
    let db = database::get().await;
    let existed_user = SysUser::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待删除的用户不存在")))?;
    let result = existed_user.delete(db).await?.rows_affected;
    Ok(ApiResponse::ok(Some(result)))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    keyword: Option<String>,
    #[serde(flatten)]
    #[validate(nested)]
    pagination: PaginationParams,
}
#[sa_check_login]
#[debug_handler]
#[sa_check_permission("user:list")]
async fn find_page(
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>,
) -> ApiResult<Page<sys_user::Model>> {
    let paginator = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Name.contains(keyword))
                    .add(sys_user::Column::MobilePhone.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(database::get().await, pagination.size);
    let total = paginator.num_items().await?;
    let users = paginator.fetch_page(pagination.page - 1).await?;
    let page = Page::from_pagination(pagination, total, users);
    Ok(ApiResponse::ok(Some(page)))
}

#[debug_handler]
#[tracing::instrument(name = "Query users", skip_all, fields(pay_method = "alipay"))]
async fn query_users() -> ApiResult<Vec<sys_user::Model>> {
    let db = database::get().await;
    tracing::warn!("出错了吗？");
    let users = SysUser::find()
        .filter(
            Condition::all()
                .add(sys_user::Column::Gender.eq("male"))
                .add(sys_user::Column::Name.starts_with("张"))
                .add(
                    Condition::any()
                        .add(sys_user::Column::Name.contains("张"))
                        .add(sys_user::Column::Name.contains("王")),
                ),
        )
        .all(db)
        .await
        .context("Fail to query users")?;
    Ok(ApiResponse::ok(Some(users)))
}
