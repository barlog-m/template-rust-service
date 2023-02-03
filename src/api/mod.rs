mod health;
pub use health::*;

mod api_error;
pub use api_error::*;

mod not_found;
pub use not_found::*;

mod layer_error;
pub use layer_error::*;

pub type ApiResult<T> = Result<axum::Json<T>, ApiError>;
