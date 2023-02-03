mod server;

pub use server::*;

use std::error::Error;
pub type TestError = Box<dyn Error + Send + Sync + 'static>;
