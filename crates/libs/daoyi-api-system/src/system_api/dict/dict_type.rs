use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    DictTypePageReqVO, DictTypeRespVO, DictTypeSaveReqVO, DictTypeUpdateReqVO, IdParams, IdsParams,
};
use daoyi_entity_system::system_service::system_dict_type_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/page", axum::routing::get(page_dict_types))
        .route("/get", axum::routing::get(get_dict_type))
        .route("/create", axum::routing::post(create_dict_type))
        .route("/update", axum::routing::put(update_dict_type))
        .route("/delete", axum::routing::delete(delete_dict_type))
        .route("/delete-list", axum::routing::delete(delete_dict_type_list))
        .route(
            "/list-all-simple",
            axum::routing::get(get_simple_dict_type_list),
        )
        .route(
            "/simple-list",
            axum::routing::get(get_simple_dict_type_list),
        )
        .route("/export-excel", axum::routing::get(page_dict_types))
}

#[debug_handler]
async fn get_simple_dict_type_list() -> RestApiResult<Vec<DictTypeRespVO>> {
    ApiResponse::success(
        system_dict_type_service::get_dict_type_list()
            .await?
            .into_iter()
            .map(|m| m.into())
            .collect(),
    )
}

#[debug_handler]
async fn delete_dict_type_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_dict_type_service::delete_dict_type_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_dict_type(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    system_dict_type_service::delete_dict_type(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_dict_type(ValidJson(vo): ValidJson<DictTypeUpdateReqVO>) -> RestApiResult<bool> {
    system_dict_type_service::update_dict_type(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_dict_type(ValidJson(vo): ValidJson<DictTypeSaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(system_dict_type_service::create_dict_type(vo).await?.id)
}

#[debug_handler]
async fn get_dict_type(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<DictTypeRespVO>> {
    ApiResponse::success(
        system_dict_type_service::get_dict_type(&id)
            .await?
            .map(|m| m.into()),
    )
}

#[debug_handler]
async fn page_dict_types(
    ValidQuery(params): ValidQuery<DictTypePageReqVO>,
) -> RestApiResult<PageResult<DictTypeRespVO>> {
    ApiResponse::success(system_dict_type_service::get_dict_type_page(&params).await?)
}
