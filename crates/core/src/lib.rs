use crate::error::Error;
use crate::settings::Settings;
use data_encoding::{BASE64, HEXLOWER};
use redis;
use regex::Regex;
use sea_orm::DatabaseConnection;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

pub mod hash;
pub mod model;
pub mod prelude;
pub mod validation;

pub mod error;
pub mod nickname;
pub mod settings;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub settings: Arc<Settings>,
    pub redis: redis::Client,
}

pub type AppResult<T> = Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub exp: i64,
}

pub fn get_file_type(hex: &str) -> &str {
    if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) {
        return "jpeg";
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {
        return "png";
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) {
        return "gif";
    }
    panic!("invalid file type")
}

pub fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec).expect("TODO: panic message");
    let encoded = BASE64.encode(&vec);
    let hex = HEXLOWER.encode(&vec);

    format!("data:image/{};base64,{}", get_file_type(&hex), encoded.replace("\r\n", ""))
}

pub fn bytes_to_base64(vec: Vec<u8>) -> String {
    let encoded = BASE64.encode(&vec);
    let hex = HEXLOWER.encode(&vec);

    format!("data:image/{};base64,{}", get_file_type(&hex), encoded.replace("\r\n", ""))
}

pub fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);

    BASE64.decode(value.as_bytes()).unwrap()
}
