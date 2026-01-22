use axum::{Router, debug_handler};
use axum::routing::{get, post, put, delete};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::enumeration::CommonStatusEnum;
use crate::service::{{ table.businessName | snake_case }}_service;
use crate::entity::{{ table.className | snake_case }};
// TODO: Import VOs
// use crate::vo::{{ table.businessName | snake_case }}_vo::{ {{ table.businessName | pascal_case }}PageReqVO, {{ table.businessName | pascal_case }}SaveReqVO, {{ table.businessName | pascal_case }}UpdateReqVo, {{ table.businessName | pascal_case }}RespVo, {{ table.businessName | pascal_case }}SimpleRespVo, {{ table.businessName | pascal_case }}IdReqVO, {{ table.businessName | pascal_case }}IdsReqVO };

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", get(get_{{ table.businessName | snake_case }}_page))
        .route("/list-all-simple", get(get_simple_{{ table.businessName | snake_case }}_list))
        .route("/create", post(create_{{ table.businessName | snake_case }}))
        .route("/update", put(update_{{ table.businessName | snake_case }}))
        .route("/get", get(get_{{ table.businessName | snake_case }}))
        .route("/delete", delete(delete_{{ table.businessName | snake_case }}))
        .route("/delete-list", delete(delete_{{ table.businessName | snake_case }}_list))
}

#[debug_handler]
async fn delete_{{ table.businessName | snake_case }}_list(
    ValidQuery(params): ValidQuery<{{ table.businessName | pascal_case }}IdsReqVO>,
) -> RestApiResult<bool> {
    {{ table.businessName | snake_case }}_service::delete_{{ table.businessName | snake_case }}_list(&params.ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_{{ table.businessName | snake_case }}(
    ValidQuery(params): ValidQuery<{{ table.businessName | pascal_case }}IdReqVO>,
) -> RestApiResult<bool> {
    {{ table.businessName | snake_case }}_service::delete_{{ table.businessName | snake_case }}(&params.id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_{{ table.businessName | snake_case }}(
    ValidQuery(params): ValidQuery<{{ table.businessName | pascal_case }}IdReqVO>,
) -> RestApiResult<Option<{{ table.businessName | pascal_case }}RespVo>> {
    ApiResponse::success(
        {{ table.businessName | snake_case }}_service::get_{{ table.businessName | snake_case }}(&params.id)
            .await?
            .map(|model| model.into()),
    )
}

#[debug_handler]
async fn update_{{ table.businessName | snake_case }}(ValidJson(vo): ValidJson<{{ table.businessName | pascal_case }}UpdateReqVo>) -> RestApiResult<bool> {
    {{ table.businessName | snake_case }}_service::update_{{ table.businessName | snake_case }}(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_{{ table.businessName | snake_case }}(ValidJson(vo): ValidJson<{{ table.businessName | pascal_case }}SaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(
        {{ table.businessName | snake_case }}_service::create_{{ table.businessName | snake_case }}(vo)
            .await?
            .id,
    )
}

#[debug_handler]
async fn get_simple_{{ table.businessName | snake_case }}_list() -> RestApiResult<Vec<{{ table.businessName | pascal_case }}SimpleRespVo>> {
    let list = {{ table.businessName | snake_case }}_service::get_{{ table.businessName | snake_case }}_list(None, Some(CommonStatusEnum::Enable)).await?;
    let list = list.into_iter().map(|item| item.into()).collect();
    ApiResponse::success(list)
}

#[debug_handler]
async fn get_{{ table.businessName | snake_case }}_page(
    ValidQuery(params): ValidQuery<{{ table.businessName | pascal_case }}PageReqVO>,
) -> RestApiResult<PageResult<{{ table.className | snake_case }}::Model>> {
    ApiResponse::success({{ table.businessName | snake_case }}_service::get_{{ table.businessName | snake_case }}_page(&params).await?)
}