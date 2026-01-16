use axum::Router;
use axum::routing;
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
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

async fn create_sms_template(
    ValidJson(req): ValidJson<SmsTemplateSaveReqVO>,
) -> RestApiResult<String> {
    let model = system_sms_template_service::create_sms_template(req).await?;
    ApiResponse::success(model.id)
}

async fn update_sms_template(
    ValidJson(req): ValidJson<SmsTemplateUpdateReqVO>,
) -> RestApiResult<bool> {
    system_sms_template_service::update_sms_template(req).await?;
    ApiResponse::success(true)
}

async fn delete_sms_template(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    system_sms_template_service::delete_sms_template(&id).await?;
    ApiResponse::success(true)
}

async fn delete_sms_template_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_sms_template_service::delete_sms_template_list(&ids).await?;
    ApiResponse::success(true)
}

async fn get_sms_template(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<SmsTemplateRespVO>> {
    let template = system_sms_template_service::get_sms_template(&id).await?;
    ApiResponse::success(template.map(Into::into))
}

async fn get_sms_template_page(
    ValidQuery(req): ValidQuery<SmsTemplatePageReqVO>,
) -> RestApiResult<daoyi_common_support::models::pagination::PageResult<SmsTemplateRespVO>> {
    let page = system_sms_template_service::get_sms_template_page(&req).await?;
    ApiResponse::success(page)
}

async fn send_sms(ValidJson(req): ValidJson<SmsTemplateSendReqVO>) -> RestApiResult<String> {
    let log_id = sms_send_service::send_single_sms_to_admin(
        &req.mobile,
        None,
        &req.template_code,
        &req.template_params,
    )
    .await?;
    ApiResponse::success(log_id)
}
