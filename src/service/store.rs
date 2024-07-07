use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{Path, Query};
use futures_util::TryFutureExt;
use redis::{AsyncCommands, RedisResult};
use sea_orm::{Condition, EntityTrait, JsonValue, ModelTrait};

use crate::AppError;
use crate::dto::list::ListParams;
use crate::service::{ApiResponse, get_conn, get_redis_conn, ListResponse, log_error, success};
use crate::state::AppState;
use crate::vo::store::Detail;

pub async fn detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<
    Json<ApiResponse<Detail>>,
    AppError
> {
    let handler_name = "store/detail";
    let conn = get_conn(&state);
    let store_vo = crate::entity::jy_main_store::Entity::find_by_id(id).into_model::<Detail>().one(conn).await.map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let store_detail = match store_vo {
        Some(s) => s,
        None => {
            return Err(AppError::notfound())
        }
    };
    Ok(success(store_detail))
}

//
pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<ListParams>,
) -> Result<
    Json<ApiResponse<ListResponse<Vec<(JsonValue, Option<JsonValue>)>>>>,
    AppError
> {
    let handler_name = "store/index";
    let conn = get_conn(&state);
    let condition = Condition::all();
    // .add(crate::entity::jy_main_store::Column::IsDeleted.eq(0));
    // .add(JyMainSite::Column::PId.eq(1));
    let test = crate::entity::jy_main_store::Entity::find().find_with_related(crate::entity::jy_main_site::Entity).into_json().all(conn).await?;
    // let selc = crate::entity::jy_main_store::Entity::find().filter(condition);
    // let record_total = selc
    //     .clone()
    //     .count(conn)
    //     .await
    //     .map_err(AppError::from)
    //     .map_err(log_error(handler_name))?;
    let page_size = params.page_size.unwrap();
    let page = params.page.unwrap();
    // let page_total = f64::ceil(record_total as f64 / page_size as f64) as usize;
    let page_total = 1usize;
    // let offset = page_size * (page - 1);
    // let list = selc
    //     .order_by_desc(crate::entity::jy_main_store::Column::Id)
    //     .limit(page_size as u64)
    //     .offset(offset as u64)
    //     .find_with_related(crate::entity::jy_main_site::Entity)
    //     // .into_model::<List>()
    //     .all(conn)
    //     .await
    //     .map_err(AppError::from)
    //     .map_err(log_error(handler_name))?;
    Ok(success(ListResponse {
        list: test,
        page,
        page_size,
        page_total,
        total: 0,
    }))
}


pub async fn redis_test(
    Extension(state): Extension<Arc<AppState>>
) -> Result<
    Json<ApiResponse<String>>,
    AppError
> {
    let handler_name = "store/redis_test";
    let redis_pool = get_redis_conn(&state);
    let mut conn = redis_pool.get().await.unwrap();
    let user: HashMap<String, String> = conn.hgetall("user").await.unwrap();
    let user_condom: String = conn.hget("user", "condom").await.unwrap();
    let user_condom2: String = match conn.hget("user", "condom2").await {
        Ok(t) => { t }
        Err(err) => { String::default() }
    };
    let result: String = conn.get("runoobkey").await.unwrap();
    Ok(success(format!("{},{},{},{:?}", result, user_condom,user_condom2, user)))
}