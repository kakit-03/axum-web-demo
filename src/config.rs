use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WebConfig {
    pub addr: String,
    pub port: i32,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub schema: String,
}

impl DatabaseConfig {
    pub fn get_link(&self)->String{
        format!("mysql://{}:{}@{}:{}/{}",&self.user,&self.password,&self.url,&self.port,&self.schema)
    }
}
#[derive(Deserialize, Debug)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct MqttConfig {
    pub url: String,
    pub qos: i32,
    pub name: String,
}
#[derive(Deserialize, Debug)]
pub struct LogConfig {
    pub path: String
}
#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub debug: bool,
    pub env:String
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub web: WebConfig,
    pub mqtt: MqttConfig,
    pub log:LogConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            // Add in `./Settings.toml`
            .add_source(config::File::with_name("Config"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(config::Environment::with_prefix("APP"))
            .build()?
            .try_deserialize::<crate::config::Config>()
    }
}
