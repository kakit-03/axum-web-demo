use std::sync::Arc;

use axum::{Extension, extract::Query, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;

use crate::{
    AppError,
    entity::jy_main_site as JyMainSite,
    state::AppState,
};
use crate::dto::list::ListParams;
use crate::dto::store::SiteParams;
use super::{ApiResponse, get_conn, JsonOrForm, ListResponse, log_error, success};

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<ListParams>,
) -> Result<
    Json<ApiResponse<ListResponse<Vec<JyMainSite::Model>>>>,
    AppError
> {
    let handler_name = "site/index";
    let conn = get_conn(&state);
    let condition = Condition::all()
        .add(JyMainSite::Column::IsDelete.eq(0));
    // .add(JyMainSite::Column::PId.eq(1));
    let selc = JyMainSite::Entity::find().filter(condition);
    let record_total = selc
        .clone()
        .count(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let page_size = params.page_size.unwrap();
    let page = params.page.unwrap();
    let page_total = f64::ceil(record_total as f64 / page_size as f64) as usize;
    let offset = page_size * (page - 1);
    let list = selc
        .order_by_desc(JyMainSite::Column::Id)
        .limit(page_size as u64)
        .offset(offset as u64)
        .all(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    Ok(success(ListResponse {
        list,
        page,
        page_size,
        page_total,
        total: record_total,
    }))
}

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    JsonOrForm(params): JsonOrForm<SiteParams>,
) -> Result<Json<ApiResponse<SiteParams>>, AppError> {
     // let handler_name = "article/add";
    // let conn = get_conn(&state);
    // JyMainSite::ActiveModel {
    //     id: NotSet,
    //     merchant_id: Set(params.merchant_id.unwrap()),
    //     category_id: Set(frm.category_id),
    //     content: Set(frm.content),
    //     ..Default::default()
    // }
    // .save(conn)
    // .await
    // .map_err(AppError::from)
    // .map_err(log_error(handler_name))?;
    Ok(success(params))
}