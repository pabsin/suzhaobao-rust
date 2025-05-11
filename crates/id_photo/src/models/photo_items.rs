use crate::dtos::wechat_dto::{CreatePhotoSpecRequest, ItemListRequest};
use crate::models::_entities::photo_items;
use crate::models::_entities::photo_items::{ActiveModel, Model};
use cuid::cuid2;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, QueryOrder, TransactionTrait};
use vegar_core::error::Error;
use vegar_core::model::query;
use vegar_core::prelude::ModelResult;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> ModelResult<Self> {
        let result = photo_items::Entity::find_by_id(id).one(db).await?;

        result.ok_or_else(|| Error::NotFound)
    }

    pub async fn find_items(
        db: &DatabaseConnection,
        params: &ItemListRequest,
    ) -> ModelResult<Vec<Self>> {
        let mut q = query::condition();
        if params.name.is_some() && params.name.as_ref().unwrap() != "" {
            q = q.like(photo_items::Column::Name, format!("%{}%", params.name.as_ref().unwrap()));
        }
        if params.class.is_some() {
            q = q.eq(photo_items::Column::Category, params.class)
        }
        let result = photo_items::Entity::find()
            .filter(q.build())
            .order_by_asc(photo_items::Column::Id)
            .paginate(db, params.page_size)
            .fetch_page(params.page - 1)
            .await?;

        Ok(result)
    }

    pub async fn create(
        db: &DatabaseConnection,
        params: &CreatePhotoSpecRequest,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let c = ActiveModel {
            id: ActiveValue::set(cuid2()),
            name: ActiveValue::set(params.name.to_string()),
            width_px: ActiveValue::set(params.width_px),
            height_px: ActiveValue::set(params.height_px),
            width_mm: ActiveValue::set(params.width_mm),
            height_mm: ActiveValue::set(params.height_mm),
            dpi: ActiveValue::set(params.dpi),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(c)
    }
}
