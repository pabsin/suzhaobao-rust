use async_trait::async_trait;
use chrono::offset::Local;
use jsonwebtoken::EncodingKey;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, DbErr,
    EntityTrait, TransactionTrait,
};
use serde::Deserialize;
use validator::Validate;

use crate::dtos::user_dto::*;
use crate::models::_entities::secrets;
use vegar_core::JwtClaims;
use vegar_core::error::Error;
use vegar_core::hash;
use vegar_core::model::{Authenticable, ModelResult, query};
use vegar_core::prelude::*;
use vegar_core::validation::Validatable;

pub use super::_entities::users::{self, ActiveModel, Entity, Model};

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

impl Validatable for ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.name.as_ref().to_owned(),
            email: self.email.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;
        if insert {
            let mut this = self;
            this.id = ActiveValue::Set(cuid::cuid2());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

#[async_trait]
impl Authenticable for Model {
    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_id(db, claims_key).await
    }
}

impl Model {
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(query::condition().eq(users::Column::Email, email).build())
            .one(db)
            .await?;
        user.ok_or_else(|| Error::NotFound)
    }

    pub async fn find_by_username(db: &DatabaseConnection, username: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(query::condition().eq(users::Column::Username, username).build())
            .one(db)
            .await?;
        user.ok_or_else(|| Error::NotFound)
    }

    pub async fn find_by_open_id(db: &DatabaseConnection, open_id: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(query::condition().eq(users::Column::OpenId, open_id).build())
            .one(db)
            .await?;
        user.ok_or_else(|| Error::NotFound)
    }

    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(query::condition().eq(users::Column::EmailVerificationToken, token).build())
            .one(db)
            .await?;
        user.ok_or_else(|| Error::NotFound)
    }

    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(query::condition().eq(users::Column::ResetToken, token).build())
            .one(db)
            .await?;
        user.ok_or_else(|| Error::NotFound)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> ModelResult<Self> {
        let user = Entity::find_by_id(id).one(db).await?;

        user.ok_or_else(|| Error::NotFound)
    }

    #[must_use]
    pub async fn verify_password(&self, password: &str, hashed_password: &str) -> bool {
        hash::verify_password(password, &hashed_password)
    }

    pub async fn update_user(
        self,
        db: &DatabaseConnection,
        params: UpdateUserInput,
    ) -> ModelResult<Model> {
        let txn = db.begin().await?;

        let mut u: ActiveModel = self.into();
        if params.name.is_some() {
            u.name = ActiveValue::set(params.name.unwrap());
        }
        if params.email.is_some() {
            u.email = ActiveValue::set(params.email.unwrap());
        }
        if params.username.is_some() {
            u.username = ActiveValue::set(params.username.unwrap());
        }

        u.avatar = ActiveValue::set(params.avatar.unwrap_or("".to_owned()));

        let r = u.update(&txn).await?;

        txn.commit().await?;

        Ok(r)
    }

    pub async fn update_user_avatar(
        self,
        db: &DatabaseConnection,
        avatar: String,
    ) -> ModelResult<Model> {
        let txn = db.begin().await?;

        let mut u: ActiveModel = self.into();
        u.avatar = ActiveValue::set(avatar);

        let r = u.update(&txn).await?;

        txn.commit().await?;

        Ok(r)
    }

    pub async fn update_user_name(
        self,
        db: &DatabaseConnection,
        name: String,
    ) -> ModelResult<Model> {
        let txn = db.begin().await?;

        let mut u: ActiveModel = self.into();
        u.name = ActiveValue::set(name);

        let r = u.update(&txn).await?;

        txn.commit().await?;

        Ok(r)
    }

    pub async fn create_with_password(
        db: &DatabaseConnection,
        params: &RegisterInput,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if users::Entity::find()
            .filter(query::condition().eq(users::Column::Email, &params.email).build())
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(Error::EntityAlreadyExists);
        }

        let mut am = ActiveModel {
            email: ActiveValue::set(params.email.to_string()),
            username: ActiveValue::set(params.username.to_string()),
            name: ActiveValue::set(params.name.to_string()),
            ..Default::default()
        };

        match params.open_id.as_ref() {
            None => {}
            Some(open_id) => am.open_id = ActiveValue::set(open_id.to_string()),
        }

        let user = am.insert(&txn).await?;
        txn.commit().await?;

        secrets::Model::create_secret_with_password(db, &user, &params.password).await?;

        Ok(user)
    }

    pub fn generate_jwt(&self, secret: &str, expiration: i64) -> ModelResult<String> {
        let claim = JwtClaims { username: self.username.to_string(), exp: expiration };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }
}

impl ActiveModel {
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.email_verification_token = ActiveValue::Set(Some(cuid::cuid2()));
        Ok(self.update(db).await?)
    }

    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.reset_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.reset_token = ActiveValue::Set(Some(cuid::cuid2()));
        Ok(self.update(db).await?)
    }

    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.email_verified_at = ActiveValue::set(Some(Local::now().into()));
        Ok(self.update(db).await?)
    }

    pub async fn set_two_factor_enable(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.two_factor_enabled = ActiveValue::set(true);
        Ok(self.update(db).await?)
    }

    pub async fn reset_two_factor_enable(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.two_factor_enabled = ActiveValue::set(false);
        Ok(self.update(db).await?)
    }
}
