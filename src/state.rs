use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use paho_mqtt::Client;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub conn: DatabaseConnection,
    pub redis: Pool<RedisConnectionManager>,
    pub mqtt: Client,
}
