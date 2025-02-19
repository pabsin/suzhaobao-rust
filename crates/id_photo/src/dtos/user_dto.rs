use crate::models::_entities::users;
use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use vegar_core::prelude::DateTime;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct LoginInput {
    /// 用户标识
    #[salvo(schema(example = "username"))]
    pub(crate) identifier: String,
    /// 密码
    #[salvo(schema(example = "abc123"))]
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct EnableToptInput {
    /// code
    #[salvo(schema(example = "123456"))]
    pub code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct UpdateUserInput {
    /// 邮箱
    #[salvo(schema(example = "517763906@qq.com"))]
    pub email: Option<String>,
    /// 名字
    #[salvo(schema(example = "chris"))]
    pub name: Option<String>,
    /// 用户名
    #[salvo(schema(example = "admin"))]
    pub username: Option<String>,
    /// 用户名
    #[salvo(schema(example = "https://www.bb.cc"))]
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct RegisterInput {
    /// 用户邮箱
    #[salvo(schema(example = "517763906@qq.com"))]
    pub email: String,
    /// 用户名
    #[salvo(schema(example = "mchriq"))]
    pub username: String,
    /// 密码
    #[salvo(schema(example = "abc123"))]
    pub password: String,
    /// 名字
    #[salvo(schema(example = "阿牛哥"))]
    pub name: String,
    /// open id
    #[salvo(schema(example = "abcd"))]
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
        }
    }
}
