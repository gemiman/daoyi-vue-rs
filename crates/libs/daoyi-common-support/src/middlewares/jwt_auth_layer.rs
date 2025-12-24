use crate::auth::{JWT, get_default_jwt};
use crate::error::ApiError;
use axum::body::Body;
use axum::http::{Request, Response, header};
use std::pin::Pin;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

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
                .transpose()?
                .ok_or_else(|| {
                    ApiError::Unauthenticated(String::from("No Authorization header"))
                })?;
            let principal = jwt.decode(token).map_err(|err| ApiError::Internal(err))?;
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

pub fn get_auth_layer() -> AsyncRequireAuthorizationLayer<JWTAuth> {
    AsyncRequireAuthorizationLayer::new(JWTAuth::new(get_default_jwt()))
}
