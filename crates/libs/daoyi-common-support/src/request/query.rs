use crate::error::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_valid::HasValidate;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Default)]
pub struct Query<T>(pub T);

impl<T> HasValidate for Query<T> {
    type Validate = T;
    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}

impl<S, T> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let query = query.replace("%5B", "[").replace("%5D", "]");
        match serde_qs::from_str(&query) {
            Ok(value) => Ok(Query(value)),
            Err(e) => {
                tracing::warn!("Query string parse error: {}", e);
                Err(ApiError::Validation(
                    format!("Invalid query parameters: {}", e).into(),
                ))
            }
        }
    }
}
