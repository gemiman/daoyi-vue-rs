use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::enumeration::CommonStatusEnum;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::system_vo::DictDataSimpleRespVO;
use daoyi_entity_system::system_service::system_dict_data_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list-all-simple", routing::get(get_simple_dict_data_list))
        .route("/simple-list", routing::get(get_simple_dict_data_list))
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
