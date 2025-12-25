use crate::configs::AppConfig;
use crate::context::HttpRequestContext;
use crate::error::ApiError;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::response::IntoResponse;
use std::pin::Pin;
use tokio::sync::OnceCell;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static THREAD_LOCAL_LAYER: OnceCell<AsyncRequireAuthorizationLayer<ThreadLocalLayer>> =
    OnceCell::const_new();

#[derive(Clone)]
pub struct ThreadLocalLayer;

impl AsyncAuthorizeRequest<Body> for ThreadLocalLayer {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send
                + 'static,
        >,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        Box::pin(async move {
            let mut context = HttpRequestContext::new();
            let auth_config = AppConfig::get().await.auth();
            let url = request.uri().path();
            let headers = request.headers();
            let token = headers
                .get(auth_config.header_key_token())
                .map(|value| -> Result<_, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization header value is not a string",
                            ))
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization header value is not a Bearer token",
                            ))
                        })?;
                    Ok(token)
                })
                .transpose()?;
            if token.is_none() && !auth_config.is_ignored_auth(url) {
                // token为空，返回错误信息
                return Err(
                    ApiError::Unauthenticated(String::from("No Authorization header"))
                        .into_response(),
                );
            }
            if let Some(token) = token {
                context.token = Some(String::from(token));
            };
            let tenant_id = headers
                .get(auth_config.header_key_tenant())
                .map(|value| -> Result<_, ApiError> {
                    let tenant_id = value.to_str().map_err(|_| {
                        ApiError::Unauthenticated(String::from(
                            "Tenant header value is not a string",
                        ))
                    })?;
                    Ok(tenant_id)
                })
                .transpose()?;
            if tenant_id.is_none() && !auth_config.is_ignored_tenant(url) {
                // Tenant 为空，返回错误信息
                return Err(
                    ApiError::Unauthenticated(String::from("No Tenant header")).into_response()
                );
            }
            if let Some(tenant_id) = tenant_id {
                context.tenant_id = Some(String::from(tenant_id));
            };
            request.extensions_mut().insert(context);
            Ok(request)
        })
    }
}

pub async fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<ThreadLocalLayer> {
    THREAD_LOCAL_LAYER
        .get_or_init(async || AsyncRequireAuthorizationLayer::new(ThreadLocalLayer))
        .await
}
