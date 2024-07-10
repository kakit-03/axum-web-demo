//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use chrono::{Utc,DateTime};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_sign_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub store_id: i32,
    pub user_id: i32,
    pub sign_time: DateTime<Utc>,
    pub status: i8,
    pub is_delete: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
