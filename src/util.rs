use axum::{async_trait, Form, Json, RequestExt};
use axum::extract::{FromRequest, Request};
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use http::header::CONTENT_TYPE;
use http::StatusCode;
use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;

/**
序列化  将chrono的DateTime<Utc>对象序列化为格式"%Y-%m-%d %H:%M:%S"的字符串
*/
pub fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = "%Y-%m-%d %H:%M:%S";
    let formatted = date.format(format).to_string();
    serializer.serialize_str(&formatted)
}
/**
反序列化 从body/form通过固定的格式"%Y-%m-%d %H:%M:%S"获取DTO时间对象
*/
pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_str = Option::<String>::deserialize(deserializer)?;

    if let Some(datetime_str) = datetime_str {
        if datetime_str.is_empty() {
            Ok(None)
        } else {
            let format = "%Y-%m-%d %H:%M:%S";
            match NaiveDateTime::parse_from_str(&datetime_str, format) {
                Ok(naive_datetime) => {
                    let datetime = Utc.from_utc_datetime(&naive_datetime);
                    Ok(Some(datetime))
                }
                Err(_) => Err(serde::de::Error::custom(format!(
                    "Failed to parse datetime: {}",
                    datetime_str
                ))),
            }
        }
    } else {
        Ok(None)
    }
}


