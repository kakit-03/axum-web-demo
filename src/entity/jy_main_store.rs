//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use chrono::{DateTime,Utc};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "jy_main_store")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub merchant_id: i32,
    pub user_id: i32,
    pub name: String,
    pub logo: String,
    pub images: String,
    pub status: i8,
    #[sea_orm(column_type = "Decimal(Some((10, 6)))")]
    pub latitude: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 6)))")]
    pub longitude: Decimal,
    pub start_time: String,
    pub end_time: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub address: String,
    pub is_deleted: i8,
    // #[serde(serialize_with  = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    // #[serde(serialize_with  = "serialize_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
