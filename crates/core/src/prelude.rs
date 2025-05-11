pub use async_trait::async_trait;

pub use chrono::NaiveDateTime as DateTime;
pub use include_dir::{Dir, include_dir};

pub use sea_orm::prelude::{Date, DateTimeWithTimeZone, Decimal, Uuid};

pub use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait,
    DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
    TransactionTrait,
};

pub use crate::model::{Authenticable, ModelResult, query};

pub mod model {
    pub use crate::model::query;
}
