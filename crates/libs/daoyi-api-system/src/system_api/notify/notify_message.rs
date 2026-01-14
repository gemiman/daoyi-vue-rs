use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, NotifyMessageMyPageReqVO, NotifyMessagePageReqVO, NotifyMessageRespVo,
    UnreadNotifyMessageListReqVO,
};
use daoyi_entity_system::system_service::system_notify_message_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/get", routing::get(get_notify_message))
        .route("/page", routing::get(get_notify_message_page))
        .route("/my-page", routing::get(get_my_notify_message_page))
        .route("/update-read", routing::put(update_notify_message_read))
        .route(
            "/update-all-read",
            routing::put(update_all_notify_message_read),
        )
        .route(
            "/get-unread-list",
            routing::get(get_unread_notify_message_list),
        )
        .route(
            "/get-unread-count",
            routing::get(get_unread_notify_message_count),
        )
}

#[debug_handler]
async fn get_unread_notify_message_list(
    ValidQuery(UnreadNotifyMessageListReqVO { size }): ValidQuery<UnreadNotifyMessageListReqVO>,
) -> RestApiResult<Vec<NotifyMessageRespVo>> {
    ApiResponse::success(
        system_notify_message_service::get_unread_notify_message_list(
            &HttpRequestContext::get_login_id_as_string()?,
            HttpRequestContext::get_user_type(),
            size,
        )
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    )
}

#[debug_handler]
async fn update_all_notify_message_read() -> RestApiResult<bool> {
    system_notify_message_service::update_all_notify_message_read(
        &HttpRequestContext::get_login_id_as_string()?,
        HttpRequestContext::get_user_type(),
    )
        .await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_notify_message_read(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_notify_message_service::update_notify_message_read(
        &ids,
        &HttpRequestContext::get_login_id_as_string()?,
        HttpRequestContext::get_user_type(),
    )
        .await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_my_notify_message_page(
    ValidQuery(params): ValidQuery<NotifyMessageMyPageReqVO>,
) -> RestApiResult<PageResult<NotifyMessageRespVo>> {
    ApiResponse::success(
        system_notify_message_service::get_my_notify_message_page(
            &params,
            &HttpRequestContext::get_login_id_as_string()?,
            HttpRequestContext::get_user_type(),
        )
            .await?,
    )
}

#[debug_handler]
async fn get_notify_message_page(
    ValidQuery(params): ValidQuery<NotifyMessagePageReqVO>,
) -> RestApiResult<PageResult<NotifyMessageRespVo>> {
    ApiResponse::success(system_notify_message_service::get_notify_message_page(&params).await?)
}

#[debug_handler]
async fn get_notify_message(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<NotifyMessageRespVo>> {
    ApiResponse::success(
        system_notify_message_service::get_notify_message(&id)
            .await?
            .map(|m| m.into()),
    )
}

#[debug_handler]
async fn get_unread_notify_message_count() -> RestApiResult<u64> {
    ApiResponse::success(
        system_notify_message_service::get_unread_notify_message_count(
            &HttpRequestContext::get_login_id_as_string()?,
            HttpRequestContext::get_user_type(),
        )
            .await?,
    )
}
