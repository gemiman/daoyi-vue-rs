use axum::extract::Query;
use axum::routing;
use axum::{Json, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::ValidJson;
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
}

async fn create_sms_channel(
    ValidJson(req): ValidJson<SmsChannelSaveReqVO>,
) -> RestApiResult<String> {
    let model = system_sms_channel_service::create_sms_channel(req).await?;
    ApiResponse::success(model.id)
}

async fn update_sms_channel(Json(req): Json<SmsChannelUpdateReqVO>) -> RestApiResult<bool> {
    system_sms_channel_service::update_sms_channel(req).await?;
    ApiResponse::success(true)
}

async fn delete_sms_channel(Query(req): Query<IdParams>) -> RestApiResult<bool> {
    system_sms_channel_service::delete_sms_channel(&req.id).await?;
    ApiResponse::success(true)
}

async fn delete_sms_channel_list(Query(req): Query<IdsParams>) -> RestApiResult<bool> {
    system_sms_channel_service::delete_sms_channel_list(&req.ids).await?;
    ApiResponse::success(true)
}

async fn get_sms_channel(Query(req): Query<IdParams>) -> RestApiResult<SmsChannelRespVO> {
    let channel = system_sms_channel_service::get_sms_channel(&req.id).await?;
    match channel {
        Some(c) => ApiResponse::success(c.into()),
        None => Ok(ApiResponse::err("短信渠道不存在")),
    }
}

async fn get_sms_channel_page(
    Query(req): Query<SmsChannelPageReqVO>,
) -> RestApiResult<daoyi_common_support::models::pagination::PageResult<SmsChannelRespVO>> {
    let page = system_sms_channel_service::get_sms_channel_page(&req).await?;
    ApiResponse::success(page)
}

async fn get_simple_sms_channel_list() -> RestApiResult<Vec<SmsChannelSimpleRespVO>> {
    let list = system_sms_channel_service::get_sms_channel_list_simple().await?;
    ApiResponse::success(list)
}
