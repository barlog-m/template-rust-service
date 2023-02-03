use super::{ApiError, ApiResult};

pub async fn not_found() -> ApiResult<()> {
    Err(ApiError::not_found())
}
