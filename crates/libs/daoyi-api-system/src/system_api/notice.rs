use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, NoticePageReqVO, NoticeRespVO, NoticeSaveReqVO, NoticeUpdateReqVO,
};
use daoyi_entity_system::system_service::system_notice_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/create", axum::routing::post(create_notice))
        .route("/update", axum::routing::put(update_notice))
        .route("/delete", axum::routing::delete(delete_notice))
        .route("/delete-list", axum::routing::delete(delete_notice_list))
        .route("/page", axum::routing::get(get_notice_page))
        .route("/get", axum::routing::get(get_notice))
        .route("/push", axum::routing::post(push))
}

#[debug_handler]
async fn push(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<bool> {
    tracing::info!("假装推送成功:{id}");
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_notice(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<NoticeRespVO>> {
    ApiResponse::success(
        system_notice_service::get_notice(&id)
            .await?
            .map(|notice| notice.into()),
    )
}

async fn get_notice_page(
    ValidQuery(params): ValidQuery<NoticePageReqVO>,
) -> RestApiResult<PageResult<NoticeRespVO>> {
    ApiResponse::success(system_notice_service::get_notice_page(&params).await?)
}

#[debug_handler]
async fn delete_notice_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_notice_service::delete_notice_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_notice(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<bool> {
    system_notice_service::delete_notice(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_notice(ValidJson(vo): ValidJson<NoticeUpdateReqVO>) -> RestApiResult<bool> {
    system_notice_service::update_notice(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_notice(ValidJson(vo): ValidJson<NoticeSaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(system_notice_service::create_notice(vo).await?.id)
}
