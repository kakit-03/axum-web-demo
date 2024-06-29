use axum::{routing::get, BoxError, Extension, Json, error_handling::HandleErrorLayer, http::{Request, Method, StatusCode, Uri}, response::IntoResponse, ServiceExt, async_trait, Form, RequestExt};
use dotenv::dotenv;
use sea_orm::Database;
use std::{
    env,
    sync::Arc,
    collections::HashMap,
    time::Duration,
};
use axum::extract::FromRequest;
use axum::response::Response;
use http::header::CONTENT_TYPE;
use tower::{Service, ServiceBuilder};
use gym::{AppError, config, router, state, service::{ApiResponse}};
use gym::err::{AppErrorItem};
use hyper::body;
use serde_json::{json, Value};
use serde_json::map::Values;
use tower_http::trace::TraceLayer;

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
struct AppConfig {
    list: Vec<String>,
}

#[tokio::main]

async fn main() {
    let cfg = config::Config::from_env().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    dotenv().ok();
    let conn = Database::connect(&cfg.database.url).await.unwrap();
    tracing::info!("Web服务监听于{}", &cfg.web.addr);
    //
    let app = router::init()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(state::AppState { conn })));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &cfg.web.addr, &cfg.web.port)).await.unwrap();
    axum::serve(listener, app)
        .await.unwrap();
}