use axum::response::IntoResponse;
use crate::error::ApiError;
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        // 直接返回 JSON 响应
        axum::Json(self).into_response()
    }
}


impl<T> ApiResponse<T> {
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        ApiResponse {
            code,
            message,
            data,
        }
    }

    pub fn ok(data: Option<T>) -> Self {
        ApiResponse::new(0, String::from("OK"), data)
    }

    pub fn err(message: String) -> Self {
        ApiResponse::new(1, message, None)
    }
}
