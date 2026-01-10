use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, PostPageReqVO, PostRespVo, PostSaveReqVO, PostSimpleRespVo,
    PostUpdateReqVo,
};
use daoyi_entity_system::system_entity::system_post;
use daoyi_entity_system::system_service::system_post_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", axum::routing::get(get_post_page))
        .route("/export-excel", axum::routing::get(get_post_page))
        .route("/list-all-simple", axum::routing::get(get_simple_post_list))
        .route("/simple-list", axum::routing::get(get_simple_post_list))
        .route("/create", axum::routing::post(create_post))
        .route("/update", axum::routing::put(update_post))
        .route("/get", axum::routing::get(get_post))
        .route("/delete", axum::routing::delete(delete_post))
        .route("/delete-list", axum::routing::delete(delete_post_list))
}

#[debug_handler]
async fn delete_post_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_post_service::delete_post_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_post(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<bool> {
    system_post_service::delete_post(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_post(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<PostRespVo>> {
    ApiResponse::success(
        system_post_service::get_post(&id)
            .await?
            .map(|post| post.into()),
    )
}

#[debug_handler]
async fn update_post(ValidJson(vo): ValidJson<PostUpdateReqVo>) -> RestApiResult<bool> {
    system_post_service::update_post(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_post(ValidJson(vo): ValidJson<PostSaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(system_post_service::create_post(vo).await?.id)
}

#[debug_handler]
async fn get_simple_post_list() -> RestApiResult<Vec<PostSimpleRespVo>> {
    // 获得岗位列表，只要开启状态的
    let list = system_post_service::get_post_list(None, Some(CommonStatusEnum::Enable)).await?;
    // 排序后，返回给前端
    let list = list.into_iter().map(|item| item.into()).collect();
    ApiResponse::success(list)
}
#[debug_handler]
async fn get_post_page(
    ValidQuery(params): ValidQuery<PostPageReqVO>,
) -> RestApiResult<Page<system_post::Model>> {
    ApiResponse::success(system_post_service::get_post_page(&params).await?)
}
