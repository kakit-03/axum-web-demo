use axum::{async_trait, Form, http::{header, HeaderMap, StatusCode}, Json, RequestExt, response::Html};
use axum::extract::FromRequest;
use axum::extract::rejection::FormRejection;
use axum::http::{Request, Response};
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum_valid::Valid;
use sea_orm::{DatabaseConnection, DeriveEntityModel};
use serde::de::DeserializeOwned;
use serde::ser::Error;
use serde_derive::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};
use crate::{state::AppState, AppError, Result, err, AppErrorType};
use crate::err::AppErrorItem;

pub mod site;


type HtmlRespon = Html<String>;
type RedirectRespon = (StatusCode, HeaderMap, ());


/// 记录错误
fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("{}: {:?}", handler_name, err.error);
        err
    })
}

fn get_conn<'a>(state: &'a AppState) -> &'a DatabaseConnection {
    &state.conn
}

pub struct JsonOrForm<T>(T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    Form<T>: FromRequest<()>,
    T: 'static + DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: axum::extract::Request, _state: &S) -> std::result::Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(|err| AppError::from_msg("反序列化失败", AppErrorType::Deserialize))?;
                return match payload.validate() {
                    Ok(_) => { Ok(Self(payload)) }
                    Err(err) => {
                        let mut s = String::new();
                        for (key, value) in err.errors().iter() {
                            match value {
                                ValidationErrorsKind::Struct(_) | ValidationErrorsKind::List(_) => {
                                    // 处理 Struct 和 List 类型的错误
                                    // 如果有共同的处理逻辑，可以在这里添加
                                }
                                ValidationErrorsKind::Field(errors) => {
                                    for error in errors {
                                        if let Some(message) = &error.message {
                                            if !s.is_empty() {
                                                s.push(',');
                                            }
                                            s.push_str(&format!("{}:{}", key, message));
                                        }
                                    }
                                }
                            }
                        }
                        // Err(AppError::from_msg(format!("{}:{}", "参数", err.errors().keys().map(|key| key.to_string()).collect::<Vec<String>>().join(",")).as_str(), AppErrorType::Deserialize))
                        Err(AppError::from_msg(format!("{}:{}", "参数", s).as_str(), AppErrorType::Deserialize))
                    }
                };
            }
            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(|_| AppError::from_msg("反序列化失败", AppErrorType::Deserialize))?;
                return match payload.validate() {
                    Ok(_) => { Ok(Self(payload)) }
                    Err(err) => {
                        let mut s = String::new();
                        for (key, value) in err.errors().iter() {
                            match value {
                                ValidationErrorsKind::Struct(_) | ValidationErrorsKind::List(_) => {
                                    // 处理 Struct 和 List 类型的错误
                                    // 如果有共同的处理逻辑，可以在这里添加
                                }
                                ValidationErrorsKind::Field(errors) => {
                                    for error in errors {
                                        if let Some(message) = &error.message {
                                            if !s.is_empty() {
                                                s.push(',');
                                            }
                                            s.push_str(&format!("{}:{}", key, message));
                                        }
                                    }
                                }
                            }
                        }
                        Err(AppError::from_msg(format!("{}:{}", "参数", s).as_str(), AppErrorType::Deserialize))
                    }
                };
            }
        }
        Err(AppError::from_msg("反序列化失败", AppErrorType::Deserialize))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiResponse<T>
{
    pub code: u16,
    pub msg: String,
    pub data: T,
}

impl<T> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        self.msg.into_response()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ListResponse<T> {
    pub list: T,
    pub total: u64,
    pub page: usize,
    pub page_size: usize,
    pub page_total: usize,
}


/**
成功返回体
 */
fn success<T>(data: T) -> axum::Json<ApiResponse<T>> {
    let r = ApiResponse {
        code: StatusCode::OK.as_u16(),
        msg: format!("{}", "请求成功"),
        data,
    };
    Json(r)
}

/**
错误返回体
 */
fn err<T>(data: T, err_msg: String) -> axum::Json<ApiResponse<T>> {
    let r = ApiResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        msg: err_msg,
        data,
    };
    Json(r)
}

fn err_response(app_error: AppError) -> axum::Json<ApiResponse<String>> {
    let msg = match app_error.error {
        AppErrorItem::Cause(err) => err.to_string(),
        AppErrorItem::Message(msg) => msg.to_string(),
    };
    let r = ApiResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        msg,
        data: "".to_string(),
    };
    Json(r)
}

fn err_msg_response(err_msg: String) -> axum::Json<ApiResponse<String>> {
    let r = ApiResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        msg: err_msg,
        data: "".to_string(),
    };
    Json(r)
}

