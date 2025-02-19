use crate::dtos::wechat_dto::{CreatePhotoDto, PhotoListRequest, UpdatePhotoDto};
use crate::models::_entities::photos;
use crate::models::_entities::photos::{ActiveModel, Model};
use cuid::cuid2;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DeleteResult, QueryOrder, TransactionTrait};
use vegar_core::error::Error;
use vegar_core::prelude::ModelResult;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> ModelResult<Self> {
        let result = photos::Entity::find_by_id(id).one(db).await?;
        result.ok_or_else(|| Error::NotFound)
    }

    pub async fn delete_user_photo_by_id(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<DeleteResult, DbErr> {
        let photo: ActiveModel = photos::Entity::find()
            .filter(photos::Column::Id.eq(id))
            .filter(photos::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find photo.".to_owned()))
            .map(Into::into)?;

        photo.delete(db).await
    }

    pub async fn find_items(
        db: &DatabaseConnection,
        params: &PhotoListRequest,
        user_id: &str,
    ) -> ModelResult<Vec<Self>> {
        let result = photos::Entity::find()
            .filter(photos::Column::UserId.eq(user_id))
            .filter(photos::Column::FilePath.ne(""))
            .order_by_asc(photos::Column::Id)
            .paginate(db, params.page_size)
            .fetch_page(params.page - 1)
            .await?;

        Ok(result)
    }

    pub async fn create_photo(
        db: &DatabaseConnection,
        params: &CreatePhotoDto,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let resume = ActiveModel {
            id: ActiveValue::set(cuid2()),
            user_id: ActiveValue::set(params.user_id.to_string()),
            name: ActiveValue::set(params.name.to_string()),
            file_name: ActiveValue::set(params.file_name.to_string()),
            file_path: ActiveValue::set(params.file_path.to_string()),
            width: ActiveValue::set(params.width),
            height: ActiveValue::set(params.height),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(resume)
    }

    pub async fn update_photo(
        self,
        db: &DatabaseConnection,
        form_data: UpdatePhotoDto,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let mut p: ActiveModel = self.into();

        if let Some(name) = form_data.name {
            p.name = ActiveValue::set(name);
        }
        if let Some(file_path) = form_data.file_path {
            p.file_path = ActiveValue::set(file_path);
        }
        if let Some(colorize_key) = form_data.colorize_key {
            p.colorize_key = ActiveValue::set(colorize_key);
        }
        if let Some(width) = form_data.width {
            p.width = ActiveValue::set(width);
        }
        if let Some(height) = form_data.height {
            p.height = ActiveValue::set(height);
        }

        let result = p.update(&txn).await?;
        txn.commit().await?;

        Ok(result)
    }
}
