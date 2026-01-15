use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{IdParams, MailLogPageReqVO, MailLogRespVO};
use daoyi_entity_system::system_service::system_mail_log_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", axum::routing::get(get_mail_log_page))
        .route("/get", axum::routing::get(get_mail_log))
}

#[debug_handler]
async fn get_mail_log_page(
    ValidQuery(params): ValidQuery<MailLogPageReqVO>,
) -> RestApiResult<PageResult<MailLogRespVO>> {
    ApiResponse::success(system_mail_log_service::get_mail_log_page(&params).await?)
}

#[debug_handler]
async fn get_mail_log(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<MailLogRespVO>> {
    ApiResponse::success(
        system_mail_log_service::get_mail_log(&id)
            .await?
            .map(Into::into),
    )
}
