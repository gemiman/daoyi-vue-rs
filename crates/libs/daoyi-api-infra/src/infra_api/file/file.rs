use axum::extract::{Multipart, Path};
use axum::response::IntoResponse;
use axum::{
    Router, debug_handler,
    routing::{delete, get, post},
};
use daoyi_common_support::app::AppState;
use daoyi_common_support::error::ApiError;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::request::valid::{ValidJson, ValidQuery};
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::infra_vo::{
    FileCreateReqVO, FilePageReqVO, FilePresignedUrlRespVO, FileRespVO, PresignedUrlReq,
};
use daoyi_common_support::vo::system_vo::{IdParams, IdsParams};
use daoyi_entity_infra::infra_service::infra_file_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/upload", post(upload_file))
        .route("/delete", delete(delete_file))
        .route("/delete-list", delete(delete_file_list))
        .route("/page", get(get_file_page))
        .route("/presigned-url", get(get_presigned_url))
        .route("/create", post(create_file_record))
        .route("/{config_id}/get/{*path}", get(get_file_content))
}

#[debug_handler]
async fn upload_file(mut multipart: Multipart) -> RestApiResult<String> {
    let mut file_content = Vec::new();
    let mut filename = String::new();
    let mut content_type = String::new();
    let mut path = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            filename = field.file_name().unwrap_or("").to_string();
            content_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();
            if let Ok(bytes) = field.bytes().await {
                file_content = bytes.to_vec();
            }
        } else if name == "path" || name == "directory" {
            // Support both parameter names
            if let Ok(txt) = field.text().await {
                if !txt.is_empty() {
                    path = Some(txt);
                }
            }
        }
    }

    if file_content.is_empty() {
        return Err(ApiError::biz("文件不能为空"));
    }

    let url = infra_file_service::create_file(filename, path, file_content, content_type).await?;
    ApiResponse::success(url)
}

#[debug_handler]
async fn delete_file(ValidQuery(IdParams { id }): ValidQuery<IdParams>) -> RestApiResult<bool> {
    infra_file_service::delete_file(&id).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn delete_file_list(
    ValidQuery(IdsParams { ids }): ValidQuery<IdsParams>,
) -> RestApiResult<bool> {
    infra_file_service::delete_file_list(&ids).await?;
    ApiResponse::success(true)
}

#[debug_handler]
async fn get_file_page(
    ValidQuery(req): ValidQuery<FilePageReqVO>,
) -> RestApiResult<Page<FileRespVO>> {
    let page = infra_file_service::get_file_page(&req).await?;
    ApiResponse::success(page)
}

#[debug_handler]
async fn get_presigned_url(
    ValidQuery(req): ValidQuery<PresignedUrlReq>,
) -> RestApiResult<FilePresignedUrlRespVO> {
    ApiResponse::success(infra_file_service::presign_put_url(req.name, req.directory).await?)
}

#[debug_handler]
async fn create_file_record(ValidJson(req): ValidJson<FileCreateReqVO>) -> RestApiResult<String> {
    ApiResponse::success(infra_file_service::create_file_from_req(req).await?)
}

#[debug_handler]
async fn get_file_content(Path((config_id, path)): Path<(String, String)>) -> impl IntoResponse {
    // path comes from wildcard, likely needs url decoding
    let decoded_path = urlencoding::decode(&path).unwrap_or(std::borrow::Cow::Borrowed(&path));

    match infra_file_service::get_file_content(&config_id, &decoded_path).await {
        Ok(bytes) => {
            // Determine content type
            let mime = mime_guess::from_path(&*decoded_path).first_or_octet_stream();
            // Return raw bytes with headers
            (
                [(axum::http::header::CONTENT_TYPE, mime.to_string())],
                bytes,
            )
                .into_response()
        }
        Err(e) => {
            // In case of error (file not found), return 404
            (
                axum::http::StatusCode::NOT_FOUND,
                format!("File not found: {}", e),
            )
                .into_response()
        }
    }
}
