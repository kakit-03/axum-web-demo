use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::util::serialize_datetime;
use chrono::{DateTime, Utc};
use sea_orm::FromQueryResult;
use crate::AppErrorType;

#[derive(Clone, Debug, Serialize,FromQueryResult)]
pub struct List {
    pub id: i32,
    pub merchant_id: i32,
    pub store_id: i32,
    pub name: String,
    // pub images: Option<Json>,
    // pub rc_config: Option<Json>,
    pub status: i8,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl From<crate::entity::jy_main_site::Model> for List {
    fn from(model: crate::entity::jy_main_site::Model) -> Self {
        Self {
            id: model.id,
            merchant_id: model.merchant_id,
            store_id: model.store_id,
            name: model.name,
            // images: model.images,
            // rc_config: model.rc_config,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.created_at,
        }
    }
}
