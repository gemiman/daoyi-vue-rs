use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_valid::{ValidRejection, ValidationRejection};

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
    #[error("参数校验错误: {0}")]
    Validation(String),
    #[error("{0}")]
    Biz(String),
    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            ValidationRejection::Valid(errors) => {
                let error_messages = format_validation_errors(&errors, None);
                let combined_message = if error_messages.is_empty() {
                    "参数验证失败".to_string()
                } else {
                    error_messages.join(" | ")
                };
                ApiError::Validation(combined_message)
            }
            ValidationRejection::Inner(errors) => errors,
        }
    }
}

fn format_validation_errors(
    errors: &validator::ValidationErrors,
    prefix: Option<&str>,
) -> Vec<String> {
    use validator::ValidationErrorsKind;
    errors
        .errors()
        .iter()
        .flat_map(|(field, errors_kind)| {
            let field_name = if let Some(p) = prefix {
                format!("{}.{}", p, field)
            } else {
                field.to_string()
            };
            match errors_kind {
                ValidationErrorsKind::Field(field_errors) => field_errors
                    .iter()
                    .map(|error| {
                        let message = error
                            .message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "验证失败".to_string());
                        format!("{}: {}", field_name, message)
                    })
                    .collect::<Vec<_>>(),
                ValidationErrorsKind::Struct(struct_errors) => {
                    format_validation_errors(struct_errors, Some(&field_name))
                }
                ValidationErrorsKind::List(list_errors) => list_errors
                    .iter()
                    .flat_map(|(index, errors)| {
                        let indexed_field = format!("{}[{}]", field_name, index);
                        format_validation_errors(errors, Some(&indexed_field))
                    })
                    .collect::<Vec<_>>(),
            }
        })
        .collect()
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        use ApiError::*;
        match self {
            NotFound => StatusCode::NOT_FOUND,
            MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Internal(_) | Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Query(_) | Path(_) | Json(_) | Validation(_) => StatusCode::BAD_REQUEST,
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
