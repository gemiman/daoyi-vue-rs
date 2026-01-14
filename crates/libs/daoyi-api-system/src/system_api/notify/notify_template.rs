use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::UserTypeEnum;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, NotifyTemplatePageReqVO, NotifyTemplateRespVo, NotifyTemplateSaveReqVO,
    NotifyTemplateSendReqVO, NotifyTemplateUpdateReqVO,
};
use daoyi_entity_system::system_service::{notify_send_service, system_notify_template_service};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/create", axum::routing::post(create_notify_template))
        .route("/update", axum::routing::put(update_notify_template))
        .route("/delete", axum::routing::delete(delete_notify_template))
        .route(
            "/delete-list",
            axum::routing::delete(delete_notify_template_list),
        )
        .route("/get", axum::routing::get(get_notify_template))
        .route("/page", axum::routing::get(get_notify_template_page))
        .route("/send-notify", axum::routing::post(send_notify))
}

#[debug_handler]
async fn send_notify(ValidJson(vo): ValidJson<NotifyTemplateSendReqVO>) -> RestApiResult<String> {
    let result = if UserTypeEnum::Member == vo.user_type {
        notify_send_service::send_single_notify_to_member(
            &vo.user_id,
            &vo.template_code,
            &vo.template_params,
        )
        .await?
    } else {
        notify_send_service::send_single_notify_to_admin(
            &vo.user_id,
            &vo.template_code,
            &vo.template_params,
        )
        .await?
    };
    ApiResponse::success(result)
}

#[debug_handler]
async fn get_notify_template_page(
    ValidQuery(params): ValidQuery<NotifyTemplatePageReqVO>,
) -> RestApiResult<PageResult<NotifyTemplateRespVo>> {
    ApiResponse::success(system_notify_template_service::get_notify_template_page(&params).await?)
}

#[debug_handler]
async fn get_notify_template(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<NotifyTemplateRespVo>> {
    ApiResponse::success(
        system_notify_template_service::get_notify_template(&id)
            .await?
            .map(|v| v.into()),
    )
}

#[debug_handler]
async fn delete_notify_template_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_notify_template_service::delete_notify_template_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_notify_template(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    system_notify_template_service::delete_notify_template(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_notify_template(
    ValidJson(vo): ValidJson<NotifyTemplateUpdateReqVO>,
) -> RestApiResult<bool> {
    system_notify_template_service::update_notify_template(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_notify_template(
    ValidJson(vo): ValidJson<NotifyTemplateSaveReqVO>,
) -> RestApiResult<String> {
    ApiResponse::success(
        system_notify_template_service::create_notify_template(vo)
            .await?
            .id,
    )
}
