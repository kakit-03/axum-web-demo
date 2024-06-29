use chrono::{DateTime, Utc};
use sea_orm::prelude::Json;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use crate::util::deserialize_datetime;

#[derive(Debug, Clone, Deserialize,Default,Validate,Serialize)]
pub struct SiteParams {
    pub id: Option<i32>,
    #[validate(range(min = 1, max = 50,message="超出了限制"))]
    pub merchant_id:Option<i32>,
    pub store_id: Option<i32>,
    pub name: Option<String>,
    pub images: Option<Json>,
    pub rc_config: Option<Json>,
    #[serde(default,deserialize_with = "deserialize_datetime")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default,deserialize_with = "deserialize_datetime")]
    pub updated_at: Option<DateTime<Utc>>
}