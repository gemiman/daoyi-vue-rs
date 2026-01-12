use axum::{Router, debug_handler};
use daoyi_common_support::app::AppState;
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::{DictTypePageReqVO, DictTypeRespVO};
use daoyi_entity_system::system_service::system_dict_type_service;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/page", axum::routing::get(page_dict_types))
}

#[debug_handler]
async fn page_dict_types(
    ValidQuery(params): ValidQuery<DictTypePageReqVO>,
) -> RestApiResult<PageResult<DictTypeRespVO>> {
    ApiResponse::success(system_dict_type_service::get_dict_type_page(&params).await?)
}
