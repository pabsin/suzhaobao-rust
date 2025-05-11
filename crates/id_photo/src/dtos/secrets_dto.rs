use sea_orm::prelude::Json;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSecretInput {
    pub id: String,
    pub user_id: String,
    pub password: String,
    pub verification_token: Option<String>,
    pub two_factor_secret: Option<String>,
    pub two_factor_backup_codes: Option<Json>,
    pub refresh_token: Option<String>,
    pub reset_token: Option<String>,
    pub webauthn: Option<Json>,
}
