use crate::app::AppState;
use crate::auth::jwt_auth::Principal;
use crate::configs::ServerConfig;
use crate::error::ApiError;
use crate::middlewares::jwt_auth_layer::get_auth_layer;
use crate::middlewares::trace_layer::LatencyOnResponse;
use crate::response::ApiResult;
use axum::extract::{DefaultBodyLimit, Request};
use axum::http::StatusCode;
use axum::{Router, debug_handler, routing};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Server { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router).await;
        let port = self.config.port();
        let listener = TcpListener::bind(format!("0.0.0.0:{port}",)).await?;
        tracing::info!("Server is listening on: http://127.0.0.1:{port}",);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    async fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let timeout =
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, self.config.timeout());
        let body_limit = DefaultBodyLimit::max(self.config.max_body_size());
        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_credentials(false)
            .max_age(self.config.max_age());
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                if let Some(principal) = request.extensions().get::<Principal>() {
                    tracing::info_span!("Api request ", id = %id, method = %method, path = %path, user_id = %principal.id, user_name = %principal.name)
                } else {
                    tracing::info_span!("Api request ", id = %id, method = %method, path = %path)
                }
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);
        let normalize_path = NormalizePathLayer::trim_trailing_slash();
        Router::new()
            .route("/", routing::get(index))
            .merge(router)
            .layer(timeout)
            .layer(body_limit)
            .layer(cors)
            .layer(normalize_path)
            .layer(tracing)
            // .route_layer(get_auth_layer().await)
            .fallback(async || -> ApiResult<()> {
                tracing::warn!("Not found");
                Err(ApiError::NotFound)
            })
            .method_not_allowed_fallback(async || -> ApiResult<()> {
                tracing::warn!("Method not allowed");
                Err(ApiError::MethodNotAllowed)
            })
            .with_state(state)
    }
}
#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}
