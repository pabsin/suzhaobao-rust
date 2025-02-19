use crate::models::_entities::user_auths;
use crate::models::_entities::user_auths::{ActiveModel, Model};
use sea_orm::entity::prelude::*;
use vegar_core::error::Error;
use vegar_core::prelude::ModelResult;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> ModelResult<Self> {
        let result = user_auths::Entity::find_by_id(id).one(db).await?;
        result.ok_or_else(|| Error::NotFound)
    }
}
