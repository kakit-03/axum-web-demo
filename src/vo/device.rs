use sea_orm::FromQueryResult;
use serde_derive::Serialize;

#[derive(Clone, Debug, Serialize,FromQueryResult,Default)]
pub struct Response {
    pub result: i32,
    pub message: String,
}