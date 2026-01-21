use axum::{
    Router,
    extract::Query,
    routing::{delete, get, post, put},
};
use daoyi_common_support::vo::infra_vo::DataSourceConfigUpdateReqVO;
use daoyi_common_support::{
    app::AppState,
    request::valid::ValidJson,
    response::{ApiResponse, RestApiResult},
    vo::infra_vo::{DataSourceConfigRespVO, DataSourceConfigSaveReqVO},
};
use daoyi_entity_infra::infra_service::infra_data_source_config_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_data_source_config))
        .route("/update", put(update_data_source_config))
        .route("/delete", delete(delete_data_source_config))
        .route("/get", get(get_data_source_config))
        .route("/list", get(get_data_source_config_list))
}

async fn create_data_source_config(
    ValidJson(req): ValidJson<DataSourceConfigSaveReqVO>,
) -> RestApiResult<String> {
    let res = infra_data_source_config_service::create_data_source_config(req).await?;
    ApiResponse::success(res)
}

async fn update_data_source_config(
    ValidJson(req): ValidJson<DataSourceConfigUpdateReqVO>,
) -> RestApiResult<()> {
    infra_data_source_config_service::update_data_source_config(req).await?;
    ApiResponse::success(())
}

#[derive(serde::Deserialize)]
struct IdReq {
    id: String,
}

async fn delete_data_source_config(Query(req): Query<IdReq>) -> RestApiResult<()> {
    infra_data_source_config_service::delete_data_source_config(&req.id).await?;
    ApiResponse::success(())
}

async fn get_data_source_config(
    Query(req): Query<IdReq>,
) -> RestApiResult<Option<DataSourceConfigRespVO>> {
    let res = infra_data_source_config_service::get_data_source_config(&req.id)
        .await?
        .map(Into::into);
    ApiResponse::success(res)
}

async fn get_data_source_config_list() -> RestApiResult<Vec<DataSourceConfigRespVO>> {
    let res = infra_data_source_config_service::get_data_source_config_list()
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    ApiResponse::success(res)
}
