use crate::auth::jwt_auth::{JWT, get_default_jwt};
use crate::configs::AppConfig;
use crate::error::ApiError;
use axum::body::Body;
use axum::http::{Request, Response, header};
use std::pin::Pin;
use tokio::sync::OnceCell;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static AUTH_LAYER: OnceCell<AsyncRequireAuthorizationLayer<JWTAuth>> = OnceCell::const_new();

#[derive(Clone)]
pub struct JWTAuth {
    jwt: &'static JWT,
}

impl JWTAuth {
    pub fn new(jwt: &'static JWT) -> Self {
        JWTAuth { jwt }
    }
}

impl AsyncAuthorizeRequest<Body> for JWTAuth {
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
        let jwt = self.jwt;
        Box::pin(async move {
            let auth_config = AppConfig::get().await.auth();
            let url = request.uri().path();
            let token = request
                .headers()
                .get(header::AUTHORIZATION)
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
            if token.is_none() && auth_config.is_ignored_url(url) {
                return Ok(request);
            }
            let token = token.ok_or_else(|| {
                ApiError::Unauthenticated(String::from("No Authorization header"))
            })?;
            let principal = jwt
                .decode(token)
                .await
                .map_err(|err| ApiError::Internal(err))?;
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

pub async fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth> {
    AUTH_LAYER
        .get_or_init(async || {
            AsyncRequireAuthorizationLayer::new(JWTAuth::new(get_default_jwt().await))
        })
        .await
}
