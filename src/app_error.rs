use backtrace::Backtrace;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    InternalError,
    ApiError,
}

#[derive(Debug)]
pub struct AppError {
    pub type_: AppErrorType,
    pub msg: String,
    pub backtrace: Backtrace,
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        Self {
            type_: AppErrorType::InternalError,
            msg: msg.to_string(),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<&String> for AppError {
    fn from(msg: &String) -> Self {
        let s: &str = &*msg;
        AppError::from(s)
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(e: serde_json::error::Error) -> Self {
        AppError::from(&format!("Serde error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_error_message() {
        let e = AppError::from("foo");
        assert_eq!("foo", e.msg);
    }

    #[test]
    fn app_error_description() {
        let e = AppError::from("foo");
        let _d = format!("{}", e);
        assert!(true);
    }
}
