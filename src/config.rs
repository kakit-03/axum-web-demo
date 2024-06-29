use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WebConfig {
    pub addr: String,
    pub port: i32,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub web: WebConfig,
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
