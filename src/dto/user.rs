use chrono::{DateTime, Utc};
use sea_orm::prelude::Json;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use crate::util::deserialize_datetime;

#[derive(Debug, Clone, Deserialize,Default,Validate,Serialize)]
pub struct UserSignParams {
    pub store_id: Option<i32>,
    pub user_id:Option<i32>
}