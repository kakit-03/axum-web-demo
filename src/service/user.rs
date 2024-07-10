use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{Path, Query};
use chrono::{DateTime};
use chrono::offset::Utc;
use futures_util::TryFutureExt;
use redis::{AsyncCommands, RedisResult};
use sea_orm::{ActiveModelTrait, Condition, DbErr, EntityTrait, JsonValue, ModelTrait, NotSet};
use sea_orm::TransactionTrait;
use sea_orm::ActiveValue::Set;
use crate::AppError;
use crate::dto::list::ListParams;
use crate::service::{ApiResponse, get_conn, get_redis_conn, JsonOrForm, ListResponse, log_error, success};
use crate::dto::user::UserSignParams;
use crate::state::AppState;
use crate::vo::store::Detail;
use crate::entity::prelude::{UserSignRecord, MainUser};
// pub async fn sign_add(
//     Extension(state): Extension<Arc<AppState>>,
//     JsonOrForm(params): JsonOrForm<UserSignParams>,
// ) -> Result<
//     Json<ApiResponse<String>>,
//     AppError
// > {
//     let handler_name = "user/sign_add";
//     let conn = get_conn(&state);
//     crate::entity::user_sign_record::ActiveModel {
//         id: NotSet,
//         store_id: Set(params.store_id.unwrap()),
//         user_id: Set(params.user_id.unwrap()),
//         sign_time: Set(Utc::now()),
//         status: Set(1),
//         is_delete: Set(1),
//         created_at: Set(Utc::now()),
//         updated_at: Set(Utc::now()),
//         ..Default::default()
//     }.save(conn)
//         .await
//         .map_err(AppError::from)
//         .map_err(log_error(handler_name))?;
//     let user_result_model: Option<crate::entity::main_user::Model> = MainUser::find_by_id(params.user_id.unwrap())
//         .one(conn)
//         .await.map_err(AppError::from)
//         .map_err(log_error(handler_name))?;
//     let mut sign_num = user_result_model.clone().unwrap().sign_num;
//     let mut user_model: crate::entity::main_user::ActiveModel = match user_result_model {
//         Some(s) => s.into(),
//         None => {
//             return Err(AppError::notfound())
//         }
//     };
//     sign_num = Option::from(sign_num.unwrap() + 1);
//     user_model.sign_num = Set(sign_num);
//     user_model.update(conn).await.map_err(AppError::from).map_err(log_error(handler_name))?;
//     Ok(success(String::from("添加成功")))
// }
pub async fn sign_add(
    Extension(state): Extension<Arc<AppState>>,
    JsonOrForm(params): JsonOrForm<UserSignParams>,
) -> Result<
    Json<ApiResponse<String>>,
    AppError
> {
    let handler_name = "user/sign_add";
    let conn = get_conn(&state).clone();
    conn.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            crate::entity::user_sign_record::ActiveModel {
                id: NotSet,
                store_id: Set(params.store_id.unwrap()),
                user_id: Set(params.user_id.unwrap()),
                sign_time: Set(Utc::now()),
                status: Set(1),
                is_delete: Set(1),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
                ..Default::default()
            }.save(txn)
                .await?;
            let user_result_model: Option<crate::entity::main_user::Model> = MainUser::find_by_id(params.user_id.unwrap())
                .one(txn)
                .await?;
            let mut sign_num = user_result_model.clone().unwrap().sign_num;
            let mut user_model: crate::entity::main_user::ActiveModel = user_result_model.unwrap().into();
            sign_num = Option::from(sign_num.unwrap() + 1);
            user_model.sign_num = Set(sign_num);
            // user_model.membership_id = Set(String::from("王道几日"));
            user_model.update(txn).await?;
            Ok(())
        })
    }).await.map_err(AppError::from).map_err(log_error(handler_name))?;
    Ok(success(String::default()))
}