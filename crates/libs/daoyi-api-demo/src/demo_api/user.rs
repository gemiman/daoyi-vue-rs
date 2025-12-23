use anyhow::Context;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::database;
use daoyi_common_support::models::pagination::{Page, PaginationParams};
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, ApiResult};
use daoyi_entity_demo::demo_entity::prelude::*;
use daoyi_entity_demo::demo_entity::sys_user;
use sea_orm::prelude::*;
use sea_orm::{Condition, QueryOrder, QueryTrait};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list", routing::get(query_users))
        .route("/page", routing::get(find_page))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    keyword: Option<String>,
    #[serde(flatten)]
    #[validate(nested)]
    pagination: PaginationParams,
}

#[debug_handler]
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
async fn query_users(State(AppState { db }): State<AppState>) -> ApiResult<Vec<sys_user::Model>> {
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
