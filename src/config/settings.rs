use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudinarySettings {
    pub cloud_name: String,
    pub api_key: String,
    pub api_secret: String,
    pub upload_preset: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MLModelSettings {
    pub model_path: String,
    pub python_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub jwt: JwtSettings,
    pub cloudinary: CloudinarySettings,
    pub ml_model: MLModelSettings,
    pub server: ServerSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize()
    }
}