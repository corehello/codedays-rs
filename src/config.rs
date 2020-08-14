use config::{self, ConfigError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct MailgunCredential {
    pub api_key: String,
    pub domain: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub mailgun: MailgunCredential,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
