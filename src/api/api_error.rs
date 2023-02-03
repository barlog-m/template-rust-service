use serde_json::json;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{app_error, AppErrorType};

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub reason: String,
    pub message: String,
    pub error_type: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: String, error_type: AppErrorType) -> Self {
        let reason = status.canonical_reason().unwrap_or("unknown reason").to_string();
        ApiError {
            status,
            reason,
            message,
            error_type: format!("{:?}", error_type)
        }
    }

    pub fn from_code(status: StatusCode) -> Self {
        let reason = status.canonical_reason().unwrap_or("unknown reason").to_string();
        let message = reason.clone();
        ApiError::new(status, message, AppErrorType::ApiError)
    }

    pub fn api_error(status: StatusCode, message: String) -> Self {
        ApiError::new(status, message, AppErrorType::ApiError)
    }

    pub fn internal_error(message: String) -> Self {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, message, AppErrorType::InternalError)
    }

    pub fn not_found() -> Self {
        ApiError::from_code(StatusCode::NOT_FOUND)
    }
}

impl From<app_error::AppError> for ApiError {
    fn from(e: app_error::AppError) -> Self {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        ApiError {
            status,
            reason: status.canonical_reason().unwrap_or("unknown reason").to_string(),
            message: e.msg,
            error_type: format!("{:?}", e.type_)
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status.as_u16();
        let status = self.status;

        let body = Json(json!({
            "status": status_code,
            "reason": self.reason,
            "message": self.message,
            "type": self.error_type,
        }));

        (status, body).into_response()
    }
}

