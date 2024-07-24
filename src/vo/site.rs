use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;
use serde::Serialize;

use crate::util::serialize_datetime;

#[derive(Clone, Debug, Serialize,FromQueryResult,Default)]
pub struct List {
    pub id: i32,
    pub merchant_id: i32,
    pub store_id: i32,
    pub store_name: Option<String>,
    pub name: String,
    // pub images: Option<Json>,
    // pub rc_config: Option<Json>,
    pub status: i8,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime<Utc>,
}
#[derive(Clone, Debug,PartialEq, Serialize,FromQueryResult)]
pub struct Detail {
    pub id: i32,
    pub merchant_id: i32,
    pub store_id: i32,
    pub name: String,
    pub store_name: String,
    pub images: Option<Json>,
    pub rc_config: Option<Json>,
    pub status: i8,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime<Utc>,
}

