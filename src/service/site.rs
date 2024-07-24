use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, extract::Query, Json};
use axum::extract::Path;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JoinType, NotSet, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;

use crate::{
    AppError,
    entity::jy_main_site as JyMainSite,
    state::AppState,
};
use crate::dto::list::ListParams;
use crate::dto::store::SiteParams;
use crate::vo::site::{Detail, List};

use super::{ApiResponse, get_conn, JsonOrForm, ListResponse, log_error, success};

type StoreColumn = crate::entity::jy_main_store::Column;
type StoreEntity = crate::entity::jy_main_store::Entity;

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<ListParams>,
) -> Result<
    Json<ApiResponse<ListResponse<Vec<List>>>>,
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
    let mut list = selc
        .order_by_desc(JyMainSite::Column::Id)
        // .select_only()
        // .columns([JyMainSite::Column::Id, JyMainSite::Column::MerchantId])
        .limit(page_size as u64)
        .offset(offset as u64)
        // .into_json()
        .into_model::<List>()
        .all(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let ids: Vec<i32> = list.iter().map(|item| item.store_id).collect();

    let store_list = StoreEntity::find()
        // when select fileds, into_model struct Detail can't be impl
        // .select_only()
        // .columns([StoreColumn::Id, StoreColumn::Name])
        .filter(Condition::all()
            .add(StoreColumn::Id.is_in(ids.clone()))
            .add(StoreColumn::IsDeleted.eq(0))
        )
        .into_model::<crate::vo::store::Detail>()
        .all(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let store_map = store_list.into_iter().map(|detail| (detail.id.clone(), detail)).collect::<HashMap<_, _>>();
    // let store_map: HashMap<_, _>  = store_map.into_iter().collect();
    for list_item in &mut list {
        if let Some(detail) = store_map.get(&list_item.store_id) {
            // 赋值部分值给 List
            list_item.store_name = Option::from((*detail).name.clone());
        }
    }
    Ok(success(ListResponse {
        list,
        page,
        page_size,
        page_total,
        total: record_total,
    }))
}

pub async fn detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Detail>>, AppError> {
    let handler_name = "site/detail";
    let conn = get_conn(&state);
    let site_vo = JyMainSite::Entity::find_by_id(id)
        .column_as(StoreColumn::Name,"store_name")
        .join_rev(JoinType::LeftJoin,StoreEntity::belongs_to(JyMainSite::Entity).from(StoreColumn::Id).to(JyMainSite::Column::StoreId).into())
        .into_model::<Detail>()
        .one(conn)
        .await.map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    Ok(success(site_vo.unwrap()))
}

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    JsonOrForm(params): JsonOrForm<SiteParams>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let handler_name = "site/add";
    let conn = get_conn(&state);
    JyMainSite::ActiveModel {
        id: NotSet,
        merchant_id: Set(params.merchant_id.unwrap()),
        store_id: Set(params.store_id.unwrap()),
        name: Set(params.name.unwrap()),
        images: Set(Option::from(params.images.unwrap())),
        rc_config: Set(Option::from(params.rc_config.unwrap())),
        ..Default::default()
    }
        .save(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    Ok(success("".to_string()))
}

pub async fn update_by_id(
    Extension(state): Extension<Arc<AppState>>,
    JsonOrForm(params): JsonOrForm<SiteParams>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let handler_name = "site/edit";
    let conn = get_conn(&state);
    let site_ac_model: Option<JyMainSite::Model> = JyMainSite::Entity::find_by_id(params.id.unwrap())
        .one(conn)
        .await.map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let mut u_model: JyMainSite::ActiveModel = match site_ac_model {
        Some(s) => s.into(),
        None => {
            return Err(AppError::notfound())
        }
    };
    u_model.name = Set(params.name.unwrap().to_owned());
    u_model.images = Set(Option::from(params.images.unwrap()).to_owned());
    u_model.rc_config = Set(Option::from(params.rc_config.unwrap()).to_owned());
    u_model.update(conn).await.map_err(AppError::from).map_err(log_error(handler_name))?;
    Ok(success("".to_string()))
}