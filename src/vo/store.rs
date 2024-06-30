use chrono::{DateTime, Utc};
use sea_orm::FromQueryResult;
use sea_orm::prelude::Decimal;
use serde_derive::Serialize;
use serde_json::Value as Json;
use crate::util::serialize_datetime;
#[derive(Clone, Debug, Serialize,FromQueryResult,Default)]
pub struct Detail {
    pub id: i32,
    pub merchant_id: i32,
    pub user_id: i32,
    pub name: String,
    pub logo: String,
    // pub site_list: Option<Vec<Json>>,
    pub images: Option<Json>,
    pub status: i8,
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub start_time: String,
    pub end_time: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub address: String,
    #[serde(serialize_with  = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with  = "serialize_datetime")]
    pub updated_at: DateTime<Utc>,
}
#[derive(Clone, Debug, Serialize,FromQueryResult)]
pub struct List {
    pub id: i32,
    pub name: String,
    pub logo: String,
    pub status: i8,
    pub start_time: String,
    pub end_time: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub address: String,
    #[serde(serialize_with  = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with  = "serialize_datetime")]
    pub updated_at: DateTime<Utc>,
}