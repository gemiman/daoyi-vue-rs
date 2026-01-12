use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{
    DictDataPageReqVO, DictDataRespVO, DictDataSaveReqVO, DictDataSimpleRespVO,
    DictDataUpdateReqVO, IdParams, IdsParams,
};
use daoyi_entity_system::system_service::system_dict_data_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list-all-simple", routing::get(get_simple_dict_data_list))
        .route("/simple-list", routing::get(get_simple_dict_data_list))
        .route("/create", routing::post(create_dict_data))
        .route("/update", routing::put(update_dict_data))
        .route("/delete", routing::delete(delete_dict_data))
        .route("/delete-list", routing::delete(delete_dict_data_list))
        .route("/page", routing::get(get_dict_data_page))
        .route("/get", routing::get(get_dict_data))
        .route("/export-excel", routing::get(get_dict_data_page))
}

#[debug_handler]
async fn get_dict_data_page(
    ValidQuery(params): ValidQuery<DictDataPageReqVO>,
) -> RestApiResult<PageResult<DictDataRespVO>> {
    ApiResponse::success(system_dict_data_service::get_dict_data_page(&params).await?)
}

#[debug_handler]
async fn get_dict_data(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<Option<DictDataRespVO>> {
    ApiResponse::success(
        system_dict_data_service::get_dict_data_by_id(&id)
            .await?
            .map(|x| x.into()),
    )
}

#[debug_handler]
async fn delete_dict_data_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    system_dict_data_service::delete_dict_data_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_dict_data(
    ValidQuery(IdParams { id }): ValidQuery<IdParams>,
) -> RestApiResult<bool> {
    system_dict_data_service::delete_dict_data(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn update_dict_data(ValidJson(vo): ValidJson<DictDataUpdateReqVO>) -> RestApiResult<bool> {
    system_dict_data_service::update_dict_data(vo).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn create_dict_data(ValidJson(vo): ValidJson<DictDataSaveReqVO>) -> RestApiResult<String> {
    ApiResponse::success(system_dict_data_service::create_dict_data(vo).await?.id)
}

#[debug_handler]
async fn get_simple_dict_data_list() -> RestApiResult<Vec<DictDataSimpleRespVO>> {
    ApiResponse::success(
        system_dict_data_service::get_dict_data_list(CommonStatusEnum::Enable, None)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect(),
    )
}
