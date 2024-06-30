pub mod config;
pub mod entity;
pub mod middleware;
pub mod err;
pub mod router;
pub mod state;
pub mod service;
pub mod dto;
pub mod util;
pub mod vo;
pub use err::{AppError, AppErrorType};

pub type Result<T> = std::result::Result<T, crate::AppError>;
