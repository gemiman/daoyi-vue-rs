use axum::extract::{Multipart, Path, Query};
use axum::response::IntoResponse;
use axum::{Json, Router, routing::{delete, get, post}};
use daoyi_common_support::app::AppState;
use daoyi_common_support::error::ApiError;
use daoyi_common_support::models::pagination::Page;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::vo::infra_vo::{
    FileCreateReqVO, FilePageReqVO, FilePresignedUrlRespVO, FileRespVO,
};
use daoyi_entity_infra::infra_service::infra_file_service;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct PresignedUrlReq {
    name: String,
    directory: Option<String>,
}

#[derive(Deserialize)]
struct DeleteReq {
    id: String,
}

#[derive(Deserialize)]
struct DeleteListReq {
    ids: String, // Comma separated
}

async fn upload_file(mut multipart: Multipart) -> RestApiResult<String> {
    let mut file_content = Vec::new();
    let mut filename = String::new();
    let mut content_type = String::new();
    let mut path = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            filename = field.file_name().unwrap_or("").to_string();
            content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
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

    match infra_file_service::create_file(filename, path, file_content, content_type).await {
        Ok(url) => ApiResponse::success(url),
        Err(e) => Err(e),
    }
}

async fn delete_file(Query(req): Query<DeleteReq>) -> RestApiResult<()> {
    match infra_file_service::delete_file(&req.id).await {
        Ok(_) => ApiResponse::success(()),
        Err(e) => Err(e),
    }
}

async fn delete_file_list(Query(req): Query<DeleteListReq>) -> RestApiResult<()> {
    let ids: Vec<String> = req.ids.split(',').map(|s| s.to_string()).collect();
    match infra_file_service::delete_file_list(&ids).await {
        Ok(_) => ApiResponse::success(()),
        Err(e) => Err(e),
    }
}

async fn get_file_page(Query(req): Query<FilePageReqVO>) -> RestApiResult<Page<FileRespVO>> {
    match infra_file_service::get_file_page(&req).await {
        Ok(page) => ApiResponse::success(page),
        Err(e) => Err(e),
    }
}

async fn get_presigned_url(Query(req): Query<PresignedUrlReq>) -> RestApiResult<FilePresignedUrlRespVO> {
    match infra_file_service::presign_put_url(req.name, req.directory).await {
        Ok(data) => ApiResponse::success(data),
        Err(e) => Err(e),
    }
}

async fn create_file_record(Json(req): Json<FileCreateReqVO>) -> RestApiResult<i64> {
    match infra_file_service::create_file_from_req(req).await {
        Ok(id) => ApiResponse::success(id),
        Err(e) => Err(e),
    }
}

async fn get_file_content(
    Path((config_id, path)): Path<(String, String)>,
) -> impl IntoResponse {
    // path comes from wildcard, likely needs url decoding
    let decoded_path = urlencoding::decode(&path).unwrap_or(std::borrow::Cow::Borrowed(&path));
    
    match infra_file_service::get_file_content(&config_id, &decoded_path).await {
        Ok(bytes) => {
            // Determine content type
            let mime = mime_guess::from_path(&*decoded_path).first_or_octet_stream();
            
            // Return raw bytes with headers
            ([(axum::http::header::CONTENT_TYPE, mime.to_string())], bytes).into_response()
        },
        Err(e) => {
             // In case of error (file not found), return 404
             (axum::http::StatusCode::NOT_FOUND, format!("File not found: {}", e)).into_response()
        }
    }
}