use axum::{routing::get, BoxError, Extension, Json, error_handling::HandleErrorLayer, http::{Request, Method, StatusCode, Uri}, response::IntoResponse, ServiceExt, async_trait, Form, RequestExt};
use dotenv::dotenv;
use sea_orm::Database;
use std::{env, sync::Arc, collections::HashMap, time::Duration, process};
use axum::extract::FromRequest;
use axum::response::Response;
use bb8_redis::RedisConnectionManager;
use chrono::{DateTime, Local};
use http::header::CONTENT_TYPE;
use tower::{Service, ServiceBuilder};
use gym::{AppError, config, router, state, service::{ApiResponse}};
use gym::err::{AppErrorItem};
use hyper::body;
use serde_json::{json, Value};
use serde_json::map::Values;
use tower_http::trace::TraceLayer;
use paho_mqtt;
use tracing::{Level, trace};
use gym::state::AppState;

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
struct AppConfig {
    list: Vec<String>,
}

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_env().unwrap();
    let gurad = gym::log::init(&cfg);
    dotenv().ok();
    tracing::info!("Web服务监听于{}", &cfg.web.addr);
    let app_state = AppState::get_state(&cfg).await;
    let extend_app = Extension(Arc::new(app_state));
    let app = router::init(extend_app.clone())
        .layer(TraceLayer::new_for_http())
        .layer(extend_app);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &cfg.web.addr, &cfg.web.port)).await.unwrap();
    axum::serve(listener, app)
        .await.unwrap();
}