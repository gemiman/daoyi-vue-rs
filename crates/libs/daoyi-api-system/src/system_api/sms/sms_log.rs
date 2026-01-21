use axum::{
    extract::Query,
    routing::get,
    Router,
};
use daoyi_common_support::{
    app::AppState,
    response::{ApiResponse, RestApiResult},
    models::pagination::PageResult,
    vo::system_vo::{SmsLogPageReqVO, SmsLogRespVO},
};
use daoyi_entity_system::system_service::system_sms_log_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", get(get_sms_log_page))
}

async fn get_sms_log_page(Query(req): Query<SmsLogPageReqVO>) -> RestApiResult<PageResult<SmsLogRespVO>> {
    let page_result = system_sms_log_service::get_sms_log_page(req).await?;

    let vo_page = page_result.map(|t| SmsLogRespVO {
        id: t.id,
        channel_id: t.channel_id,
        channel_code: t.channel_code.to_string(),
        template_id: t.template_id,
        template_code: t.template_code,
        template_type: t.template_type.to_string(),
        template_content: t.template_content,
        template_params: t.template_params,
        api_template_id: t.api_template_id,
        mobile: t.mobile,
        user_id: t.user_id,
        user_type: t.user_type,
        send_status: t.send_status,
        send_time: t.send_time,
        api_send_code: t.api_send_code,
        api_send_msg: t.api_send_msg,
        api_serial_no: t.api_serial_no,
        api_request_id: t.api_request_id,
        receive_status: t.receive_status,
        receive_time: t.receive_time,
        create_time: t.create_time,
    });

    ApiResponse::success(vo_page)
}
