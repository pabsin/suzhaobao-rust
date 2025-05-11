use config::{Config, ConfigError, Environment, File};
use educe::Educe;
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::env;
use tokio::sync::RwLock;

#[derive(Clone, Debug, Educe, Deserialize)]
#[allow(unused)]
#[serde(default)]
#[educe(Default)]
pub struct Database {
    #[educe(Default = "sqlite://data.sqlite?mode=rwc")]
    pub dsn: String,
}

#[derive(Clone, Debug, Educe, Deserialize)]
#[allow(unused)]
#[serde(default)]
#[educe(Default)]
pub struct Redis {
    #[educe(Default = "redis://default@127.0.0.1:6379/")]
    pub dsn: String,
}

#[derive(Debug, Clone, Educe, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct Auth {
    #[educe(Default = "abcdefghijklmnopqrst")]
    pub jwt_secret: String,
}

#[derive(Clone, Debug, Educe, Deserialize)]
#[allow(unused)]
#[serde(default)]
#[educe(Default)]
pub struct App {
    pub debug: bool,
    #[educe(Default = "http://127.0.0.1:5800")]
    pub host: String,
}

#[derive(Clone, Debug, Educe, Deserialize)]
#[allow(unused)]
#[serde(default)]
#[educe(Default)]
pub struct S3 {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    #[educe(Default = "http://127.0.0.1:9000")]
    pub endpoint: String,
    #[educe(Default = "suzhaobao")]
    pub bucket: String,
}

#[derive(Clone, Debug, Educe, Deserialize)]
#[allow(unused)]
#[serde(default)]
#[educe(Default)]
pub struct IdPhotoSetting {
    pub safety_uri: String,
    pub zjz_uri: String,
    pub matting_uri: String,
    pub colorize_uri: String,
    pub cartoon_uri: String,
    pub human_matting_model: String,
    pub face_detect_model: String,
    pub matting_model: String,
}

#[derive(Clone, Debug, Educe, Deserialize)]
#[allow(unused)]
#[serde(default)]
#[educe(Default)]
pub struct Settings {
    pub app: App,
    pub database: Database,
    pub redis: Redis,
    pub auth: Auth,
    pub s3: S3,
    pub id_photo_setting: IdPhotoSetting,
}

lazy_static! {
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::new().unwrap());
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default").required(false))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app").separator("_"))
            .build()?;
        s.try_deserialize()
    }
}
