use axum::extract::Query;
use axum::routing;
use axum::{Json, Router};
use daoyi_common_support::app::AppState;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, SmsTemplatePageReqVO, SmsTemplateRespVO, SmsTemplateSaveReqVO,
    SmsTemplateSendReqVO, SmsTemplateUpdateReqVO,
};
use daoyi_entity_system::system_service::{sms_send_service, system_sms_template_service};

pub fn create_template_router() -> Router<AppState> {
    Router::new()
        .route("/create", routing::post(create_sms_template))
        .route("/update", routing::put(update_sms_template))
        .route("/delete", routing::delete(delete_sms_template))
        .route("/delete-list", routing::delete(delete_sms_template_list))
        .route("/get", routing::get(get_sms_template))
        .route("/page", routing::get(get_sms_template_page))
        .route("/send-sms", routing::post(send_sms))
}

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
