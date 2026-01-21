use axum::http::header;
use axum::{
    Router,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::vo::infra_vo::{DbTableListReq, TableIdParam};
use daoyi_common_support::{
    app::AppState,
    error::ApiError,
    models::pagination::PageResult,
    request::valid::ValidJson,
    response::{ApiResponse, RestApiResult},
    vo::infra_vo::{
        CodegenCreateListReqVO, CodegenDetailRespVO, CodegenPreviewRespVO, CodegenTablePageReqVO,
        CodegenTableRespVO, CodegenUpdateReqVO, DatabaseTableRespVO,
    },
};
use daoyi_entity_infra::infra_service::{
    infra_codegen_engine::CodegenEngine, infra_codegen_service,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/db/table/list", get(get_database_table_list))
        .route("/table/page", get(get_codegen_table_page))
        .route("/create-list", post(create_codegen_list))
        .route("/update", put(update_codegen))
        .route("/delete", delete(delete_codegen))
        .route("/sync-from-db", put(sync_codegen_from_db))
        .route("/detail", get(get_codegen_detail))
        .route("/preview", get(preview_codegen))
        .route("/download", get(download_codegen))
}

async fn get_database_table_list(
    ValidQuery(req): ValidQuery<DbTableListReq>,
) -> RestApiResult<Vec<DatabaseTableRespVO>> {
    let res = infra_codegen_service::get_database_table_list(
        &req.data_source_config_id,
        req.name,
        req.comment,
    )
        .await?;
    ApiResponse::success(res)
}

async fn get_codegen_table_page(
    ValidQuery(params): ValidQuery<CodegenTablePageReqVO>,
) -> RestApiResult<PageResult<CodegenTableRespVO>> {
    ApiResponse::success(infra_codegen_service::get_codegen_table_page(&params).await?)
}

async fn create_codegen_list(
    ValidJson(req): ValidJson<CodegenCreateListReqVO>,
) -> RestApiResult<Vec<String>> {
    let res = infra_codegen_service::create_codegen_list(req).await?;
    ApiResponse::success(res)
}

async fn update_codegen(ValidJson(req): ValidJson<CodegenUpdateReqVO>) -> RestApiResult<()> {
    infra_codegen_service::update_codegen(req).await?;
    ApiResponse::success(())
}

async fn delete_codegen(
    ValidQuery(TableIdParam { table_id }): ValidQuery<TableIdParam>,
) -> RestApiResult<()> {
    infra_codegen_service::delete_codegen(&table_id).await?;
    ApiResponse::success(())
}

async fn sync_codegen_from_db(
    ValidQuery(TableIdParam { table_id }): ValidQuery<TableIdParam>,
) -> RestApiResult<()> {
    infra_codegen_service::sync_codegen_from_db(&table_id).await?;
    ApiResponse::success(())
}

async fn get_codegen_detail(
    ValidQuery(TableIdParam { table_id }): ValidQuery<TableIdParam>,
) -> RestApiResult<CodegenDetailRespVO> {
    let table = infra_codegen_service::get_codegen_table(&table_id)
        .await?
        .map(Into::into);
    let columns = infra_codegen_service::get_codegen_columns(&table_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    ApiResponse::success(CodegenDetailRespVO { table, columns })
}

async fn preview_codegen(
    ValidQuery(TableIdParam { table_id }): ValidQuery<TableIdParam>,
) -> RestApiResult<Vec<CodegenPreviewRespVO>> {
    let table = infra_codegen_service::get_codegen_table(&table_id).await?;
    let columns = infra_codegen_service::get_codegen_columns(&table_id).await?;

    if let Some(t) = table {
        let codes = CodegenEngine::execute(&t, &columns, &[], &[]);
        let resp: Vec<CodegenPreviewRespVO> = codes
            .into_iter()
            .map(|(path, code)| CodegenPreviewRespVO {
                file_path: path,
                code,
            })
            .collect();
        ApiResponse::success(resp)
    } else {
        Err(ApiError::biz("生成预览失败"))
    }
}

async fn download_codegen(
    ValidQuery(TableIdParam { table_id }): ValidQuery<TableIdParam>,
) -> impl IntoResponse {
    match infra_codegen_service::download_codegen(&table_id).await {
        Ok(data) => (
            [
                (header::CONTENT_TYPE, "application/zip"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"codegen.zip\"",
                ),
            ],
            data,
        )
            .into_response(),
        Err(e) => {
            let resp: ApiResponse<()> = ApiResponse::err(e.to_string());
            resp.into_response()
        }
    }
}
