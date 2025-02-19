pub mod query;
use crate::error::Error;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct ModelValidation {
    pub code: String,
    pub message: Option<String>,
}

#[allow(clippy::module_name_repetitions)]
pub type ModelResult<T, E = Error> = Result<T, E>;

#[async_trait]
pub trait Authenticable: Clone {
    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self>;
}
