use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method not Allowed")]
    MethodNotAllowed,
    #[error("Database Error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("查询参数错误: {0}")]
    Query(#[from] QueryRejection),
    #[error("路径参数错误: {0}")]
    Path(#[from] PathRejection),
    #[error("请求体参数错误: {0}")]
    Json(#[from] JsonRejection),
    #[error("{0}")]
    Biz(String),
    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        use ApiError::*;
        match self {
            NotFound => StatusCode::NOT_FOUND,
            MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Internal(_) | Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Query(_) | Path(_) | Json(_) => StatusCode::BAD_REQUEST,
            Biz(_) => StatusCode::OK,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = axum::Json(ApiResponse::<()>::err(self.to_string()));
        (status_code, body).into_response()
    }
}
