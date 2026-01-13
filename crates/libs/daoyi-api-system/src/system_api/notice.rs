use axum::extract::State;
use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::UserTypeEnum;
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
async fn push(
    State(AppState { ws_sender, .. }): State<AppState>,
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    // 1. 获取通知详情
    let notice = system_notice_service::get_notice(&id).await?;

    if let Some(notice_do) = notice {
        // 2. 通过 websocket 推送给在线的管理员用户
        // 对标 Java: webSocketSenderApi.sendObject(UserTypeEnum.ADMIN.getValue(), "notice-push", notice);
        ws_sender
            .send_by_user_type(
                UserTypeEnum::Admin,
                "notice-push",
                NoticeRespVO::from(notice_do),
            )
            .await;
        ApiResponse::success(true)
    } else {
        // ApiResponse::err 返回的是 Self，而 RestApiResult<bool> 实际上是 Result<ApiResponse<bool>, ApiError>
        // 查看 RestApiResult 的定义。
        Ok(ApiResponse::err("公告不存在"))
    }
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
