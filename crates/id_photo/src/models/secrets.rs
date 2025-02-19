use crate::models::_entities::secrets;
use crate::models::_entities::secrets::{ActiveModel, Model};
use crate::models::users::users;
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, DbErr,
    EntityTrait, TransactionTrait,
};
use vegar_core::error::Error;
use vegar_core::hash;
use vegar_core::model::{ModelResult, query};

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let mut this = self;
            this.id = ActiveValue::Set(cuid::cuid2());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {
    pub async fn get_user_secret(db: &DatabaseConnection, user_id: &str) -> ModelResult<Self> {
        let user = secrets::Entity::find()
            .filter(query::condition().eq(secrets::Column::UserId, user_id).build())
            .one(db)
            .await?;
        user.ok_or_else(|| Error::NotFound)
    }

    pub async fn create_secret_with_password(
        db: &DatabaseConnection,
        user: &users::Model,
        password: &str,
    ) -> ModelResult<Self> {
        let password_hash = hash::hash_password(&password).map_err(|e| Error::Any(e.into()))?;

        let txn = db.begin().await?;

        let secret = ActiveModel {
            user_id: ActiveValue::set(user.id.clone()),
            password: ActiveValue::set(password_hash),
            ..Default::default()
        }
        .insert(db)
        .await?;

        txn.commit().await?;

        Ok(secret)
    }

    pub async fn update_secret(
        db: &DatabaseConnection,
        user: &users::Model,
        password: &str,
    ) -> ModelResult<Self> {
        let password_hash = hash::hash_password(&password).map_err(|e| Error::Any(e.into()))?;

        let txn = db.begin().await?;

        let secret = ActiveModel {
            user_id: ActiveValue::set(user.id.clone()),
            password: ActiveValue::set(password_hash),
            ..Default::default()
        }
        .insert(db)
        .await?;

        txn.commit().await?;

        Ok(secret)
    }

    pub async fn set_two_factor_secret(
        self,
        db: &DatabaseConnection,
        secret: &str,
    ) -> ModelResult<Model> {
        let txn = db.begin().await?;
        let mut pear: ActiveModel = self.into();

        pear.two_factor_secret = ActiveValue::Set(Option::from(secret.to_string()));
        let p = pear.update(db).await?;

        txn.commit().await?;

        Ok(p)
    }

    pub async fn set_two_factor_backup_codes(
        self,
        db: &DatabaseConnection,
        codes: serde_json::Value,
    ) -> ModelResult<Model> {
        let mut pear: ActiveModel = self.into();
        pear.two_factor_backup_codes = ActiveValue::Set(Option::from(codes));
        let p = pear.update(db).await?;

        Ok(p)
    }
}

impl ActiveModel {
    pub async fn remove_two_factor_infos(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.two_factor_secret = ActiveValue::set(None);
        self.two_factor_backup_codes = ActiveValue::set(None);
        Ok(self.update(db).await?)
    }

    pub async fn remove_refresh_token(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.refresh_token = ActiveValue::set(None);
        Ok(self.update(db).await?)
    }
}
