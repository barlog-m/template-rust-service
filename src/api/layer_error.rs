use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use tower::BoxError;
use super::ApiError;

pub async fn layer_error_handler(error: BoxError) -> impl IntoResponse {
    tracing::debug!("layer error: {:?}", error);

    if error.is::<tower::timeout::error::Elapsed>() {
        return ApiError::from_code(StatusCode::REQUEST_TIMEOUT);
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return ApiError::from_code(StatusCode::SERVICE_UNAVAILABLE);
    }

    ApiError::internal_error(format!("Unhandled internal error: {}", error))
}
