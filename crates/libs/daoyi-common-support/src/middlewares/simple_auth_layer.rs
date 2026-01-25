use crate::configs::AppConfig;
use crate::context::HttpRequestContext;
use crate::enumeration::{APP_API, UserTypeEnum};
use crate::error::ApiError;
use crate::{auth, id_util};
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Request, Response};
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::net::SocketAddr;
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
            context.tracing_id = Some(Arc::new(id_util::xid()));
            // 获取连接信息
            context.user_ip = Some(Arc::new(get_real_client_ip(&request)));
            // 获取 User-Agent
            let headers = request.headers();
            let user_agent = headers
                .get("User-Agent")
                .and_then(|value| value.to_str().ok())
                .map(|s| s.to_string())
                .unwrap_or_default();
            context.user_agent = Some(Arc::new(user_agent));
            context.request_method = Some(Arc::new(request.method().to_string()));
            context.request_url = Some(Arc::new(request.uri().to_string()));

            // Check if tenant is enabled
            let auth_config = AppConfig::get().auth();
            let url = request.uri().path();
            let user_type = if url.starts_with(APP_API) {
                UserTypeEnum::Member
            } else {
                UserTypeEnum::Admin
            };
            context.user_type = Some(user_type);
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
            // 如果 Header 中没有 Token，尝试从 URL Query 参数获取 (适配 WebSocket)
            let token = if token.is_none() {
                if let Some(query) = request.uri().query() {
                    serde_qs::from_str::<std::collections::HashMap<String, String>>(query)
                        .ok()
                        .and_then(|params| params.get("token").cloned())
                } else {
                    None
                }
            } else {
                token.map(String::from)
            };
            // 此时 token 为 Option<String>
            let token = token.as_deref();
            let is_ignored_auth_url = auth_config.is_ignored_auth(url);
            if token.is_none() && !is_ignored_auth_url {
                // token为空，返回错误信息
                return Err(ApiError::unauthenticated("No Authorization token").into_response());
            }
            let mut token_tenant_id: Option<Arc<String>> = None;
            if let Some(token) = token
                && !is_ignored_auth_url
            {
                let token_info = auth::check_token(token).await?;
                let t_id = Arc::new(token_info.tenant_id);
                token_tenant_id = Some(t_id.clone());
                context.tenant_id = Some(t_id);
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
                    if token_tenant_id.as_str() != tenant_id {
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

// 添加此函数来获取真实客户端IP
fn get_real_client_ip<B>(request: &Request<B>) -> String {
    let headers = vec![
        "X-Forwarded-For",
        "X-Real-IP",
        "Proxy-Client-IP",
        "WL-Proxy-Client-IP",
        "HTTP_CLIENT_IP",
        "HTTP_X_FORWARDED_FOR",
    ];
    // 检查 头部（可能包含多个IP，第一个通常是原始客户端IP）
    for header in headers {
        if let Some(forwarded_for) = request.headers().get(header) {
            if let Ok(forwarded_str) = forwarded_for.to_str() {
                // X-Forwarded-For 可能包含多个IP地址，以逗号分隔
                if let Some(client_ip) = forwarded_str.split(',').next() {
                    let trimmed_ip = client_ip.trim();
                    // 验证是否为有效的IP地址格式
                    if let Ok(_) = trimmed_ip.parse::<std::net::IpAddr>() {
                        return trimmed_ip.to_string();
                    }
                }
            }
        }
    }
    // 如果以上头部都不存在或无效，则返回连接的远程地址
    if let Some(ConnectInfo(addr)) = request.extensions().get::<ConnectInfo<SocketAddr>>() {
        addr.ip().to_string()
    } else {
        "unknown".to_string()
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
