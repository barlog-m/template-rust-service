pub mod model;
pub mod logic;
pub mod api;

mod router;
pub use router::*;

mod app_error;
pub use app_error::*;

pub type AppResult<T> = Result<T, AppError>;
