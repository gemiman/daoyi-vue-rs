use axum::extract::State;
use axum::response::IntoResponse;
use axum::{debug_handler, routing, Router};
use daoyi_common_support::app;
use daoyi_common_support::app::AppState;
use daoyi_entity_demo::demo_entity::prelude::*;
use daoyi_entity_demo::demo_entity::sys_user;
use sea_orm::prelude::*;
use sea_orm::Condition;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(query_users));
    app::run(Some(env!("CARGO_PKG_NAME")), router).await
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}

#[debug_handler]
async fn query_users(State(AppState { db }): State<AppState>) -> impl IntoResponse {
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
        .unwrap();
    axum::Json(users)
}
