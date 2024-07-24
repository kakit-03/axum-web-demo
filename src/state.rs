use std::process;
use std::time::Duration;
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use paho_mqtt::Client;
use sea_orm::{Database, DatabaseConnection};
use crate::config::Config;

pub struct AppState {
    pub conn: DatabaseConnection,
    pub redis: Pool<RedisConnectionManager>,
    pub mqtt: Client,
}

impl AppState {
    pub async fn get_state(cfg: &Config) -> AppState {
        // let cfg = crate::config::Config::from_env().unwrap();
        //mqtt start
        //mqtt config
        let create_opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(&cfg.mqtt.url)
            .client_id(&cfg.mqtt.name)
            .finalize();
        // Create a client.
        let mqtt = paho_mqtt::Client::new(create_opts).unwrap_or_else(|err| {
            println!("Error creating the client: {:?}", err);
            process::exit(1);
        });
        // Define the set of options for the connection.
        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .finalize();
        // Connect and wait for it to complete or fail.
        if let Err(e) = mqtt.connect(conn_opts) {
            tracing::error!("{}: {:?}", "mqtt_connect", e.to_string());
        }
        let redis_url:&str = &cfg.redis.url.as_str();
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        let redis = Pool::builder().build(manager).await.unwrap();
        let database_url = &cfg.database.get_link();
        let conn = Database::connect(database_url).await.unwrap();
        AppState {
            conn,
            redis,
            mqtt,
        }
    }
}
