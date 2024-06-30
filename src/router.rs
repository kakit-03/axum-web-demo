use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::http::StatusCode;
use crate::middleware::auth;
use crate::{AppError, service};
pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/site", get(service::site::index))
        .route("/site/add", post(service::site::add))
        .route("/site/detail/:id", get(service::site::detail))
        .route("/site/edit", post(service::site::update_by_id)).layer(middleware::from_fn(auth::kakit_authorization_middleware))
        .route("/", get(|| async {}))
        .route("/signin", post(auth::sign_in))
        .fallback(handler_404)
    // .route(
    //     "/category/add",
    //     get(handler::category::add_ui).post(handler::category::add),
    // )
    // .route(
    //     "/category/edit/:id",
    //     get(handler::category::edit_ui).post(handler::category::edit),
    // )
    // .route("/category/del/:id", get(handler::category::del))
    // .route("/category/del/:id/:real", get(handler::category::del))
    // .route("/category/articles/:id", get(handler::category::articles))
    // .route("/article", get(handler::site::index))
    // .route(
    //     "/article/add",
    //     get(handler::site::add_ui).post(handler::site::add),
    // )
    // .route("/article/tags", get(handler::site::list_with_tags))
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, AppError::route_not_found())
}