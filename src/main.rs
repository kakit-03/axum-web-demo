use axum::{routing::get, BoxError, Extension, Json, error_handling::HandleErrorLayer, http::{Request, Method, StatusCode, Uri}, response::IntoResponse, ServiceExt, async_trait, Form, RequestExt};
use dotenv::dotenv;
use sea_orm::Database;
use std::{env, sync::Arc, collections::HashMap, time::Duration, process};
use axum::extract::FromRequest;
use axum::response::Response;
use bb8_redis::RedisConnectionManager;
use http::header::CONTENT_TYPE;
use tower::{Service, ServiceBuilder};
use gym::{AppError, config, router, state, service::{ApiResponse}};
use gym::err::{AppErrorItem};
use hyper::body;
use serde_json::{json, Value};
use serde_json::map::Values;
use tower_http::trace::TraceLayer;
use paho_mqtt;
use tracing::trace;

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
    //mqtt start
    //mqtt config
    let create_opts = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(cfg.mqtt.url)
        .client_id(cfg.mqtt.name)
        .finalize();
    // Create a client.
    let cli = paho_mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });
    // Define the set of options for the connection.
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();
    // Connect and wait for it to complete or fail.
    if let Err(e) = cli.connect(conn_opts) {
        tracing::error!("{}: {:?}", "mqtt_connect", e.to_string());
    }
    let manager = RedisConnectionManager::new(cfg.redis.url.as_str()).unwrap();
    let pool = bb8_redis::bb8::Pool::builder().build(manager).await.unwrap();
    let database_url = cfg.database.get_link();
    let conn = Database::connect(database_url).await.unwrap();
    tracing::info!("Web服务监听于{}", &cfg.web.addr);
    let app_state = Arc::new(state::AppState { conn, redis: pool,mqtt:cli });
    let extend_app = Extension(app_state);
    let app = router::init(extend_app.clone())
        .layer(TraceLayer::new_for_http())
        .layer(extend_app);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &cfg.web.addr, &cfg.web.port)).await.unwrap();
    axum::serve(listener, app)
        .await.unwrap();
}