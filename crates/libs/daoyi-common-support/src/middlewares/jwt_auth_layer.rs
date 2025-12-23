use crate::auth::{get_default_jwt, JWT};
use tower_http::auth::AsyncRequireAuthorizationLayer;

pub struct JWTAuth {
    jwt: &'static JWT,
}

impl JWTAuth {
    pub fn new(jwt: &'static JWT) -> Self {
        JWTAuth { jwt }
    }
}

pub fn get_auth_layer() -> AsyncRequireAuthorizationLayer<JWTAuth> {
    AsyncRequireAuthorizationLayer::new(JWTAuth::new(get_default_jwt()))
}
