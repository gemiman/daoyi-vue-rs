use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, MailAccountPageReqVO, MailAccountRespVO, MailAccountSaveReqVO,
    MailAccountUpdateReqVO,
};
use daoyi_entity_system::system_service::system_mail_account_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/create", axum::routing::post(create_mail_account))
        .route("/update", axum::routing::put(update_mail_account))
        .route("/delete", axum::routing::delete(delete_mail_account))
        .route(
            "/delete-list",
            axum::routing::delete(delete_mail_account_list),
        )
        .route("/get", axum::routing::get(get_mail_account))
        .route("/page", axum::routing::get(get_mail_account_page))
        .route(
            "/list-all-simple",
            axum::routing::get(get_simple_mail_account_list),
        )
        .route(
            "/simple-list",
            axum::routing::get(get_simple_mail_account_list),
        )
}

#[debug_handler]
async fn get_simple_mail_account_list() -> RestApiResult<Vec<MailAccountRespVO>> {
    ApiResponse::success(
        system_mail_account_service::get_mail_account_list()
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    )
}

#[debug_handler]
async fn get_mail_account_page(
    ValidQuery(params): ValidQuery<MailAccountPageReqVO>,
) -> RestApiResult<PageResult<MailAccountRespVO>> {
    ApiResponse::success(system_mail_account_service::get_mail_account_page(&params).await?)
}

#[debug_handler]
async fn get_mail_account(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<MailAccountRespVO>> {
    ApiResponse::success(
        system_mail_account_service::get_mail_account(&id)
            .await?
            .map(Into::into),
    )
}

#[debug_handler]
async fn delete_mail_account_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_mail_account_service::delete_mail_account_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_mail_account(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    system_mail_account_service::delete_mail_account(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_mail_account(
    ValidJson(vo): ValidJson<MailAccountUpdateReqVO>,
) -> RestApiResult<bool> {
    system_mail_account_service::update_mail_account(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_mail_account(
    ValidJson(vo): ValidJson<MailAccountSaveReqVO>,
) -> RestApiResult<String> {
    ApiResponse::success(
        system_mail_account_service::create_mail_account(vo)
            .await?
            .id,
    )
}
