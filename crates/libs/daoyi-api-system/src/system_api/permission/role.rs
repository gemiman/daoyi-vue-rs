use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    IdParams, IdsParams, RolePageReqVO, RoleRespVO, RoleSaveReqVO, RoleUpdateReqVO,
};
use daoyi_entity_system::system_entity::system_role;
use daoyi_entity_system::system_service::system_role_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", axum::routing::get(get_role_page))
        .route("/export-excel", axum::routing::get(get_role_page))
        .route("/list-all-simple", axum::routing::get(get_simple_role_list))
        .route("/simple-list", axum::routing::get(get_simple_role_list))
        .route("/create", axum::routing::post(create_role))
        .route("/update", axum::routing::put(update_role))
        .route("/get", axum::routing::get(get_role))
        .route("/delete", axum::routing::delete(delete_role))
        .route("/delete-list", axum::routing::delete(delete_role_list))
}

#[debug_handler]
async fn delete_role_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_role_service::delete_role_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_role(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<bool> {
    system_role_service::delete_role(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_role(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<RoleRespVO> {
    ApiResponse::success(system_role_service::get_role_by_id(&id).await?.into())
}

#[debug_handler]
async fn update_role(ValidJson(vo): ValidJson<RoleUpdateReqVO>) -> RestApiResult<bool> {
    system_role_service::update_role(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_role(ValidJson(vo): ValidJson<RoleSaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(system_role_service::create_role(vo, None).await?.id)
}

#[debug_handler]
async fn get_simple_role_list() -> RestApiResult<Vec<RoleRespVO>> {
    let list =
        system_role_service::get_role_list_by_status(Some(vec![CommonStatusEnum::Enable])).await?;
    ApiResponse::success(list.into_iter().map(|item| item.into()).collect())
}

#[debug_handler]
async fn get_role_page(
    ValidQuery(params): ValidQuery<RolePageReqVO>,
) -> RestApiResult<Page<system_role::Model>> {
    ApiResponse::success(system_role_service::get_role_page(&params).await?)
}
