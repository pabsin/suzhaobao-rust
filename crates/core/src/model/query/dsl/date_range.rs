use chrono::NaiveDateTime;
use sea_orm::ColumnTrait;

use super::{ConditionBuilder, with};
pub struct DateRangeBuilder<T: ColumnTrait> {
    col: T,
    condition_builder: ConditionBuilder,
    from_date: Option<NaiveDateTime>,
    to_date: Option<NaiveDateTime>,
}

impl<T: ColumnTrait> DateRangeBuilder<T> {
    pub const fn new(condition_builder: ConditionBuilder, col: T) -> Self {
        Self { col, condition_builder, from_date: None, to_date: None }
    }

    #[must_use]
    pub fn dates(self, from: Option<&NaiveDateTime>, to: Option<&NaiveDateTime>) -> Self {
        Self {
            col: self.col,
            condition_builder: self.condition_builder,
            from_date: from.copied(),
            to_date: to.copied(),
        }
    }

    #[must_use]
    pub fn from(self, from: &NaiveDateTime) -> Self {
        Self {
            col: self.col,
            condition_builder: self.condition_builder,
            from_date: Some(*from),
            to_date: self.to_date,
        }
    }

    #[must_use]
    pub fn to(self, to: &NaiveDateTime) -> Self {
        Self {
            col: self.col,
            condition_builder: self.condition_builder,
            from_date: self.from_date,
            to_date: Some(*to),
        }
    }

    pub fn build(self) -> ConditionBuilder {
        let con = match (self.from_date, self.to_date) {
            (None, None) => self.condition_builder.condition,
            (None, Some(to)) => self.condition_builder.condition.add(self.col.lt(to)),
            (Some(from), None) => self.condition_builder.condition.add(self.col.gt(from)),
            (Some(from), Some(to)) => {
                self.condition_builder.condition.add(self.col.between(from, to))
            }
        };
        with(con)
    }
}
