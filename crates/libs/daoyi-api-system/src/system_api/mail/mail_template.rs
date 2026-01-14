use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::context::HttpRequestContext;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, MailTemplatePageReqVO, MailTemplateRespVO, MailTemplateSaveReqVO,
    MailTemplateSendReqVO, MailTemplateUpdateReqVO,
};
use daoyi_entity_system::system_service::{mail_send_service, system_mail_template_service};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/create", axum::routing::post(create_mail_template))
        .route("/update", axum::routing::put(update_mail_template))
        .route("/delete", axum::routing::delete(delete_mail_template))
        .route(
            "/delete-list",
            axum::routing::delete(delete_mail_template_list),
        )
        .route("/get", axum::routing::get(get_mail_template))
        .route("/page", axum::routing::get(get_mail_template_page))
        .route(
            "/list-all-simple",
            axum::routing::get(get_simple_template_list),
        )
        .route("/simple-list", axum::routing::get(get_simple_template_list))
        .route("/send-mail", axum::routing::post(send_single_mail_to_admin))
}

#[debug_handler]
async fn send_single_mail_to_admin(
    ValidJson(params): ValidJson<MailTemplateSendReqVO>,
) -> RestApiResult<String> {
    ApiResponse::success(
        mail_send_service::send_single_mail_to_admin(
            &HttpRequestContext::get_login_id_as_string()?,
            &params.to_mails,
            &params.cc_mails,
            &params.bcc_mails,
            &params.template_code,
            &params.template_params,
        )
        .await?,
    )
}

#[debug_handler]
async fn get_simple_template_list() -> RestApiResult<Vec<MailTemplateRespVO>> {
    ApiResponse::success(
        system_mail_template_service::get_mail_template_list()
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    )
}

#[debug_handler]
async fn get_mail_template_page(
    ValidQuery(params): ValidQuery<MailTemplatePageReqVO>,
) -> RestApiResult<PageResult<MailTemplateRespVO>> {
    ApiResponse::success(system_mail_template_service::get_mail_template_page(&params).await?)
}
#[debug_handler]
async fn get_mail_template(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<MailTemplateRespVO>> {
    ApiResponse::success(
        system_mail_template_service::get_mail_template(&id)
            .await?
            .map(Into::into),
    )
}

#[debug_handler]
async fn delete_mail_template_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_mail_template_service::delete_mail_template_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_mail_template(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    system_mail_template_service::delete_mail_template(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_mail_template(
    ValidJson(vo): ValidJson<MailTemplateUpdateReqVO>,
) -> RestApiResult<bool> {
    system_mail_template_service::update_mail_template(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_mail_template(
    ValidJson(vo): ValidJson<MailTemplateSaveReqVO>,
) -> RestApiResult<String> {
    ApiResponse::success(
        system_mail_template_service::create_mail_template(vo)
            .await?
            .id,
    )
}
