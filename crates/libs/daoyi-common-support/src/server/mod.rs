use crate::app::AppState;
use crate::auth::Principal;
use crate::configs::ServerConfig;
use crate::error::ApiError;
use crate::middlewares::simple_auth_layer;
use crate::middlewares::trace_layer::LatencyOnResponse;
use crate::response::RestApiResult;
use axum::extract::{DefaultBodyLimit, Request};
use axum::http::StatusCode;
use axum::{Router, debug_handler, middleware, routing};
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
        let result = axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await;

        // 服务关闭后执行清理工作
        tracing::info!("Server shutdown initiated, cleaning up resources...");
        self.cleanup().await?;
        tracing::info!("Server has been gracefully shut down");

        result?;
        Ok(())
    }

    async fn cleanup(&self) -> anyhow::Result<()> {
        use crate::{database, redis_utils};

        // 关闭数据库连接池
        if let Err(e) = database::shutdown().await {
            tracing::error!("Failed to close database connection pool: {}", e);
        }

        // 关闭 Redis 连接池
        if let Err(e) = redis_utils::shutdown().await {
            tracing::error!("Failed to close Redis connection pool: {}", e);
        }

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
            .layer(normalize_path)
            .layer(tracing)
            .layer(middleware::from_fn(
                simple_auth_layer::thread_local_middleware,
            ))
            .route_layer(simple_auth_layer::get_auth_layer().await)
            .fallback(async || -> RestApiResult<()> {
                tracing::warn!("Not found");
                Err(ApiError::NotFound)
            })
            .method_not_allowed_fallback(async || -> RestApiResult<()> {
                tracing::warn!("Method not allowed");
                Err(ApiError::MethodNotAllowed)
            })
            .layer(cors)
            .with_state(state)
    }
}
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal, shutting down gracefully...");
        },
    }
}
#[debug_handler]
async fn index() -> &'static str {
    "Hello, Daoyi Vue Rust!"
}
