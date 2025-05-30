//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use crate::dtos::user_dto::MemberShipDto;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub username: String,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub avatar: String,
    pub two_factor_enabled: bool,
    pub open_id: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,

    pub reset_token: Option<String>,
    pub reset_sent_at: Option<DateTimeWithTimeZone>,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<DateTimeWithTimeZone>,
    pub email_verified_at: Option<DateTimeWithTimeZone>,

    #[sea_orm(ignore)]
    pub membership: MemberShipDto,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::secrets::Entity")]
    Secrets,
}

impl Related<super::secrets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Secrets.def()
    }
}
