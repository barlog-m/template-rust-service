use std::time::Duration;
use axum::{
    Router,
    routing::get,
    error_handling::HandleErrorLayer,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::api;

pub fn app() -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(api::layer_error_handler))
        .load_shed()
        .concurrency_limit(1024)
        .timeout(Duration::from_secs(60))
        .layer(TraceLayer::new_for_http());

    Router::new()
        .route("/", get(|| async {}))
        .nest("/health", api::health_router())
        .fallback(api::not_found)
        .layer(middleware_stack)
}

