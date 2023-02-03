use crate::model::HealthInfo;
use crate::api::ApiResult;
use crate::logic::health::*;

use axum::{routing::get, Router};

pub async fn health_get() -> ApiResult<HealthInfo> {
    let status = health_status().await?;
    Ok(status.into())
}

pub fn health_router() -> Router {
    return Router::new()
        .route("/", get(health_get))
}
