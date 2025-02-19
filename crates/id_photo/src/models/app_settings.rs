use crate::models::_entities::app_settings;
use crate::models::_entities::app_settings::{ActiveModel, Model};
use sea_orm::entity::prelude::*;
use vegar_core::error::Error;
use vegar_core::model::query;
use vegar_core::prelude::ModelResult;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> ModelResult<Self> {
        let result = app_settings::Entity::find_by_id(id).one(db).await?;

        result.ok_or_else(|| Error::NotFound)
    }

    pub async fn find_by_app_id(db: &DatabaseConnection, app_id: &str) -> ModelResult<Self> {
        let result = app_settings::Entity::find()
            .filter(query::condition().eq(app_settings::Column::AppId, app_id).build())
            .one(db)
            .await?;
        result.ok_or_else(|| Error::NotFound)
    }
}
