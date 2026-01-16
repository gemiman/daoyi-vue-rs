use axum::Router;
use axum::routing;
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{IdParams, IdsParams};
use daoyi_common_support::vo::system_vo::{
    SmsChannelPageReqVO, SmsChannelRespVO, SmsChannelSaveReqVO, SmsChannelSimpleRespVO,
    SmsChannelUpdateReqVO,
};
use daoyi_entity_system::system_service::system_sms_channel_service;

pub fn create_channel_router() -> Router<AppState> {
    Router::new()
        .route("/create", routing::post(create_sms_channel))
        .route("/update", routing::put(update_sms_channel))
        .route("/delete", routing::delete(delete_sms_channel))
        .route("/delete-list", routing::delete(delete_sms_channel_list))
        .route("/get", routing::get(get_sms_channel))
        .route("/page", routing::get(get_sms_channel_page))
        .route("/simple-list", routing::get(get_simple_sms_channel_list))
        .route(
            "/list-all-simple",
            routing::get(get_simple_sms_channel_list),
        )
}

async fn create_sms_channel(
    ValidJson(req): ValidJson<SmsChannelSaveReqVO>,
) -> RestApiResult<String> {
    let model = system_sms_channel_service::create_sms_channel(req).await?;
    ApiResponse::success(model.id)
}

async fn update_sms_channel(
    ValidJson(req): ValidJson<SmsChannelUpdateReqVO>,
) -> RestApiResult<bool> {
    system_sms_channel_service::update_sms_channel(req).await?;
    ApiResponse::success(true)
}

async fn delete_sms_channel(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    system_sms_channel_service::delete_sms_channel(&id).await?;
    ApiResponse::success(true)
}

async fn delete_sms_channel_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_sms_channel_service::delete_sms_channel_list(&ids).await?;
    ApiResponse::success(true)
}

async fn get_sms_channel(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<SmsChannelRespVO> {
    let channel = system_sms_channel_service::get_sms_channel(&id).await?;
    match channel {
        Some(c) => ApiResponse::success(c.into()),
        None => Ok(ApiResponse::err("短信渠道不存在")),
    }
}

async fn get_sms_channel_page(
    ValidQuery(req): ValidQuery<SmsChannelPageReqVO>,
) -> RestApiResult<PageResult<SmsChannelRespVO>> {
    let page = system_sms_channel_service::get_sms_channel_page(&req).await?;
    ApiResponse::success(page)
}

async fn get_simple_sms_channel_list() -> RestApiResult<Vec<SmsChannelSimpleRespVO>> {
    let list = system_sms_channel_service::get_sms_channel_list_simple().await?;
    ApiResponse::success(list)
}
