use crate::auth;
use crate::configs::AppConfig;
use crate::context::HttpRequestContext;
use crate::error::ApiError;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::pin::Pin;
use std::sync::Arc;
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
            // Check if tenant is enabled
            let auth_config = AppConfig::get().auth();
            let url = request.uri().path();
            let headers = request.headers();
            let is_ignored_tenant = auth_config.is_ignored_tenant(url);
            context.ignore_tenant = Some(is_ignored_tenant);
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
                context.tenant_id = token_tenant_id.clone().map(Arc::new);
                context.token = Some(Arc::new(String::from(token)));
                context.login_id = Some(Arc::new(String::from(token_info.user_id)));
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
            if tenant_id.is_none() && !is_ignored_tenant {
                // Tenant 为空，返回错误信息
                return Err(ApiError::unauthenticated("No Tenant header").into_response());
            }
            if let Some(tenant_id) = tenant_id
                && !is_ignored_tenant
            {
                if let Some(token_tenant_id) = token_tenant_id {
                    if token_tenant_id != tenant_id {
                        return Err(
                            ApiError::unauthenticated("Token tenant id mismatch").into_response()
                        );
                    }
                } else {
                    auth::check_tenant_id(tenant_id).await?;
                }
                context.tenant_id = Some(Arc::new(String::from(tenant_id)));
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
    let context = request
        .extensions()
        .get::<HttpRequestContext>()
        .cloned()
        .unwrap_or_else(HttpRequestContext::new);

    HttpRequestContext::scope(context, || next.run(request)).await
}
