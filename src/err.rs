use std::fmt::Display;
use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::Form;

use crate::service::ApiResponse;

#[derive(Debug)]
pub enum AppErrorType {
    Database,
    Template,
    Notfound,
    Deserialize,
    FORBIDDEN,
    UNAUTHORIZED,
    REDISERROR,
}

type Cause = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum AppErrorItem {
    Message(String),
    Cause(Cause),
}

#[derive(Debug)]
pub struct AppError {
    pub types: AppErrorType,
    pub error: AppErrorItem,
}

impl AppError {
    pub fn new(types: AppErrorType, error: AppErrorItem) -> Self {
        Self { types, error }
    }
    pub fn from_err(cause: Cause, types: AppErrorType) -> Self {
        Self::new(types, AppErrorItem::Cause(cause))
    }
    pub fn from_msg(msg: &str, types: AppErrorType) -> Self {
        Self::new(types, AppErrorItem::Message(msg.to_string()))
    }
    pub fn auth_err(msg: &str) -> Self {
        Self::new(AppErrorType::FORBIDDEN, AppErrorItem::Message(msg.to_string()))
    }
    pub fn un_auth_err(msg: &str) -> Self {
        Self::new(AppErrorType::UNAUTHORIZED, AppErrorItem::Message(msg.to_string()))
    }
    pub fn notfound() -> Self {
        Self::from_msg("不存在的记录", AppErrorType::Notfound)
    }
    pub fn route_not_found() -> Self {
        Self::from_msg("路由不存在", AppErrorType::Notfound)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl serde::ser::Error for AppError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        AppError {
            types: AppErrorType::Deserialize, // 自定义错误类型
            error: AppErrorItem::Message(msg.to_string()),
        }
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Database)
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::from_err(Box::new(err), AppErrorType::Deserialize)
    }
}
impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::from_err(Box::new(err), AppErrorType::REDISERROR)
    }
}


impl From<axum::http::Error> for AppError {
    fn from(err: axum::http::Error) -> Self {
        AppError::from_err(Box::new(err), AppErrorType::Deserialize)
    }
}


impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = match self.error {
            AppErrorItem::Cause(err) => err.to_string(),
            AppErrorItem::Message(msg) => msg.to_string(),
        };
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(
                ApiResponse {
                    code: axum::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    msg,
                    data: "".to_string(),
                }
            )
        ).into_response()
    }
}
