use anyhow::Context;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::response::{ApiResponse, ApiResult};
use daoyi_entity_demo::demo_entity::prelude::*;
use daoyi_entity_demo::demo_entity::sys_user;
use sea_orm::Condition;
use sea_orm::prelude::*;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/list", routing::get(query_users))
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
        .all(&db)
        .await
        .context("Fail to query users")?;
    Ok(ApiResponse::ok(Some(users)))
}
