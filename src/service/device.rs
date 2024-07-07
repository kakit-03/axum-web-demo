use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Json, response};
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;

use crate::AppError;
use crate::dto::device::{DeviceResultParam, HeartParam, QrCodeResult};
use crate::service::{ApiResponse, get_conn, JsonOrForm, log_error, success};
use crate::state::AppState;
use crate::vo::device::Response;

pub async fn heart(
    Extension(state): Extension<Arc<AppState>>,
    JsonOrForm(params): JsonOrForm<HeartParam>,
) -> Result<Json<Response>, AppError> {
    let handler_name = "device/heart";
    let conn = get_conn(&state);
    tracing::error!("{}: {:?}", handler_name, params);

    let response = Response {
        result: 0,
        message: "测试成功".to_string(),
    };
    Ok(Json(response))
}
pub async fn get_device_result(
    Extension(state): Extension<Arc<AppState>>,
    Json(params): Json<DeviceResultParam>,
) -> Result<Json<Response>, AppError> {
    let handler_name = "device/get_device_result";
    // let conn = get_conn(&state);
    tracing::error!("{}: {:?}", handler_name, params);

    let response = Response {
        result: 0,
        message: "测试成功".to_string(),
    };
    Ok(Json(response))
}
pub async fn get_qr_code_result(
    Extension(state): Extension<Arc<AppState>>,
    Json(params): Json<QrCodeResult>,
) -> Result<Json<Response>, AppError> {
    let handler_name = "device/get_qr_code_result";
    // let conn = get_conn(&state);
    tracing::error!("{}: {:?}", handler_name, params);

    let response = Response {
        result: 0,
        message: "测试成功".to_string(),
    };
    Ok(Json(response))
}