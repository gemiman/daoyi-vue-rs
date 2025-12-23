use crate::app::AppState;
use crate::configs::ServerConfig;
use crate::error::ApiError;
use crate::logger::log;
use crate::middlewares::trace_layer::LatencyOnResponse;
use crate::response::ApiResult;
use axum::extract::Request;
use axum::{Router, debug_handler, routing};
use std::net::SocketAddr;
use tokio::net::TcpListener;
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
        log::info!("Server is listening on: http://127.0.0.1:{port}",);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    async fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("Api request ", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);
        Router::new()
            .route("/", routing::get(index))
            .merge(router)
            .layer(tracing)
            .fallback(async || -> ApiResult<()> {
                log::warn!("Not found");
                Err(ApiError::NotFound)
            })
            .method_not_allowed_fallback(async || -> ApiResult<()> {
                log::warn!("Method not allowed");
                Err(ApiError::MethodNotAllowed)
            })
            .with_state(state)
    }
}
#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}
