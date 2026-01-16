use axum::extract::Query;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{IdParams, IdsParams};
use daoyi_common_support::vo::system_vo::{
    SmsChannelPageReqVO, SmsChannelRespVO, SmsChannelSaveReqVO, SmsChannelSimpleRespVO,
    SmsChannelUpdateReqVO, SmsTemplatePageReqVO, SmsTemplateRespVO, SmsTemplateSaveReqVO,
    SmsTemplateSendReqVO, SmsTemplateUpdateReqVO,
};
use daoyi_entity_system::system_service::{
    sms_send_service, system_sms_channel_service, system_sms_template_service,
};

pub fn create_channel_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_sms_channel))
        .route("/update", put(update_sms_channel))
        .route("/delete", delete(delete_sms_channel))
        .route("/delete-list", delete(delete_sms_channel_list))
        .route("/get", get(get_sms_channel))
        .route("/page", get(get_sms_channel_page))
        .route("/simple-list", get(get_simple_sms_channel_list))
}

pub fn create_template_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_sms_template))
        .route("/update", put(update_sms_template))
        .route("/delete", delete(delete_sms_template))
        .route("/delete-list", delete(delete_sms_template_list))
        .route("/get", get(get_sms_template))
        .route("/page", get(get_sms_template_page))
        .route("/send-sms", post(send_sms))
}

// ==================== SmsChannel Handlers ====================

async fn create_sms_channel(Json(req): Json<SmsChannelSaveReqVO>) -> RestApiResult<String> {
    let id = system_sms_channel_service::create_sms_channel(req).await?;
    ApiResponse::success(id)
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

// ==================== SmsTemplate Handlers ====================

async fn create_sms_template(Json(req): Json<SmsTemplateSaveReqVO>) -> RestApiResult<String> {
    let id = system_sms_template_service::create_sms_template(req).await?;
    ApiResponse::success(id)
}

async fn update_sms_template(Json(req): Json<SmsTemplateUpdateReqVO>) -> RestApiResult<bool> {
    system_sms_template_service::update_sms_template(req).await?;
    ApiResponse::success(true)
}

async fn delete_sms_template(Query(req): Query<IdParams>) -> RestApiResult<bool> {
    system_sms_template_service::delete_sms_template(&req.id).await?;
    ApiResponse::success(true)
}

async fn delete_sms_template_list(Query(req): Query<IdsParams>) -> RestApiResult<bool> {
    system_sms_template_service::delete_sms_template_list(&req.ids).await?;
    ApiResponse::success(true)
}

async fn get_sms_template(Query(req): Query<IdParams>) -> RestApiResult<SmsTemplateRespVO> {
    let template = system_sms_template_service::get_sms_template(&req.id).await?;
    match template {
        Some(t) => ApiResponse::success(t.into()),
        None => Ok(ApiResponse::err("短信模板不存在")),
    }
}

async fn get_sms_template_page(
    Query(req): Query<SmsTemplatePageReqVO>,
) -> RestApiResult<daoyi_common_support::models::pagination::PageResult<SmsTemplateRespVO>> {
    let page = system_sms_template_service::get_sms_template_page(&req).await?;
    ApiResponse::success(page)
}

async fn send_sms(Json(req): Json<SmsTemplateSendReqVO>) -> RestApiResult<String> {
    let log_id = sms_send_service::send_single_sms_to_admin(
        &req.mobile,
        None,
        &req.template_code,
        &req.template_params,
    )
    .await?;
    ApiResponse::success(log_id)
}
