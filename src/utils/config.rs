use config::ConfigError;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Serverconfig {
    pub host: String,
    pub port: i32,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub pass: String,
    pub server: String,
    pub dbname: String,
}

#[derive(Clone, Deserialize)]
pub struct TwitterConfig{
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: Serverconfig,
    pub twitter: TwitterConfig,
    pub mongodb: DatabaseConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
