use crate::models::_entities::users;
use salvo::macros::Extractible;
use serde::{Deserialize, Serialize};
use vegar_core::prelude::DateTime;

#[derive(Clone, Debug, Serialize, Deserialize, Extractible)]
pub struct LoginInput {
    pub identifier: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Extractible)]
pub struct EnableToptInput {
    pub code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Extractible)]
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Extractible)]
pub struct RegisterInput {
    pub email: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub open_id: Option<String>,
}

#[derive(Clone, Debug, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct MemberShipDto {
    pub days_remaining: u64,
    pub end_date: DateTime,
    pub expired_days: u64,
    pub has_membership: bool,
    pub is_expired: bool,
    pub membership_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub id: String,
    pub avatar: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime,
    pub email_verified_at: Option<DateTime>,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            id: user.id.clone(),
            avatar: user.avatar.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            created_at: user.created_at.naive_local(),
            email_verified_at: user.email_verified_at.map(|x| x.naive_local()),
        }
    }
}
