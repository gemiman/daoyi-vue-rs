use crate::auth;
use crate::configs::AppConfig;
use crate::context::HttpRequestContext;
use crate::error::ApiError;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::middleware::Next;
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
                            ApiError::unauthenticated("Authorization header value is not a string")
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| {
                            ApiError::unauthenticated(
                                "Authorization header value is not a Bearer token",
                            )
                        })?;
                    Ok(token)
                })
                .transpose()?;
            if token.is_none() && !auth_config.is_ignored_auth(url) {
                // token为空，返回错误信息
                return Err(ApiError::unauthenticated("No Authorization header").into_response());
            }
            let mut token_tenant_id = None;
            if let Some(token) = token {
                let token_info = auth::check_token(token).await?;
                token_tenant_id = Some(token_info.tenant_id);
                context.token = Some(String::from(token));
            };
            let tenant_id = headers
                .get(auth_config.header_key_tenant())
                .map(|value| -> Result<_, ApiError> {
                    let tenant_id = value.to_str().map_err(|_| {
                        ApiError::unauthenticated("Tenant header value is not a string")
                    })?;
                    Ok(tenant_id)
                })
                .transpose()?;
            if tenant_id.is_none() && !auth_config.is_ignored_tenant(url) {
                // Tenant 为空，返回错误信息
                return Err(ApiError::unauthenticated("No Tenant header").into_response());
            }
            if let Some(tenant_id) = tenant_id {
                if let Some(token_tenant_id) = token_tenant_id {
                    if token_tenant_id != tenant_id {
                        return Err(
                            ApiError::unauthenticated("Token tenant id mismatch").into_response()
                        );
                    }
                } else {
                    auth::check_tenant_id(tenant_id).await?;
                }
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

pub async fn thread_local_middleware(request: Request<Body>, next: Next) -> Response<Body> {
    if let Some(context) = request.extensions().get::<HttpRequestContext>() {
        HttpRequestContext::set_current(context.clone());
    } else {
        HttpRequestContext::set_current(HttpRequestContext::new());
    }
    let response = next.run(request).await;
    HttpRequestContext::clear();
    response
}
