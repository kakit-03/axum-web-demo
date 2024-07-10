use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json};
use crate::AppError;
use crate::service::{ApiResponse, get_mqtt_client, get_redis_conn, log_error, success};
use crate::state::AppState;

pub async fn test(
    Extension(state): Extension<Arc<AppState>>
) -> Result<
    Json<ApiResponse<String>>,
    AppError
> {
    let handler_name = "store/redis_test";
    let redis_pool = get_redis_conn(&state);
    let mqtt_client = get_mqtt_client(&state);
    let msg = paho_mqtt::Message::new("rust/mqtt", "ceshi", 1);
    mqtt_client.publish(msg).map_err(AppError::from).map_err(log_error(handler_name))?;
    Ok(success("".to_string()))
}