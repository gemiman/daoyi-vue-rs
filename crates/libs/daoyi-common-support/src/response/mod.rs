use crate::error::ApiError;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

pub type RestApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    success: bool,
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
    pub fn new<M: AsRef<str>>(code: i32, message: M, data: Option<T>) -> Self {
        ApiResponse {
            code,
            message: String::from(message.as_ref()),
            data,
            success: code == 0,
        }
    }

    pub fn success(data: T) -> RestApiResult<T> {
        Ok(Self::ok(Some(data)))
    }

    pub fn ok(data: Option<T>) -> Self {
        ApiResponse::new(0, "ok", data)
    }

    pub fn err<M: AsRef<str>>(message: M) -> Self {
        ApiResponse::new(1, message, None)
    }
}
