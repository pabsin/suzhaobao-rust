use sea_orm::{
    ColumnTrait, Condition, Value,
    sea_query::{IntoCondition, Order},
};
use serde::{Deserialize, Serialize};

mod date_range;

pub struct ConditionBuilder {
    condition: Condition,
}
/// Enum representing sorting directions, with serialization and deserialization
/// support.
#[derive(Debug, Deserialize, Serialize)]
pub enum SortDirection {
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "asc")]
    Asc,
}

impl SortDirection {
    /// Returns the corresponding `Order` enum variant based on the current
    /// `SortDirection`.
    #[must_use]
    pub const fn order(&self) -> Order {
        match self {
            Self::Desc => Order::Desc,
            Self::Asc => Order::Asc,
        }
    }
}

#[must_use]
pub fn condition() -> ConditionBuilder {
    ConditionBuilder { condition: Condition::all() }
}

#[must_use]
pub const fn with(condition: Condition) -> ConditionBuilder {
    ConditionBuilder { condition }
}

/// Builder query condition
impl ConditionBuilder {
    /// where condition the given column equals the given value
    #[must_use]
    pub fn eq<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.eq(value)))
    }

    /// where condition the given column not equals the given value
    #[must_use]
    pub fn ne<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.ne(value)))
    }

    /// where condition the given column greater than the given value
    #[must_use]
    pub fn gt<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.gt(value)))
    }

    /// where condition the given column greater than or equal to the given
    #[must_use]
    pub fn gte<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.gte(value)))
    }

    /// where condition the given column smaller than to the given
    #[must_use]
    pub fn lt<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.lt(value)))
    }

    /// where condition the given column smaller than or equal to the given
    #[must_use]
    pub fn lte<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.lte(value)))
    }

    /// where condition the given column between the given values
    #[must_use]
    pub fn between<T: ColumnTrait, V: Into<Value>>(self, col: T, a: V, b: V) -> Self {
        with(self.condition.add(col.between(a, b)))
    }

    /// where condition the given column not between the given values
    #[must_use]
    pub fn not_between<T: ColumnTrait, V: Into<Value>>(self, col: T, a: V, b: V) -> Self {
        with(self.condition.add(col.not_between(a, b)))
    }

    /// where condition the given column like given values
    #[must_use]
    pub fn like<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.like(a)))
    }

    /// where condition the given column not like given values
    #[must_use]
    pub fn not_like<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.not_like(a)))
    }

    /// where condition the given column start with given values
    #[must_use]
    pub fn starts_with<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.starts_with(a)))
    }

    /// where condition the given column end with given values
    #[must_use]
    pub fn ends_with<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.ends_with(a)))
    }

    /// where condition the given column end with given values
    #[must_use]
    pub fn contains<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.contains(a)))
    }

    /// where condition the given column is null
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_null<T: ColumnTrait>(self, col: T) -> Self {
        with(self.condition.add(col.is_null()))
    }

    /// where condition the given column is not null
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_null<T: ColumnTrait>(self, col: T) -> Self {
        with(self.condition.add(col.is_not_null()))
    }

    /// where condition the given column is in
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_in<T: ColumnTrait, V: Into<Value>, I: IntoIterator<Item = V>>(
        self,
        col: T,
        values: I,
    ) -> Self {
        with(self.condition.add(col.is_in(values)))
    }

    /// where condition the given column is not in
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_in<T: ColumnTrait, V: Into<Value>, I: IntoIterator<Item = V>>(
        self,
        col: T,
        values: I,
    ) -> Self {
        with(self.condition.add(col.is_not_in(values)))
    }

    /// where condition the given column is not null
    #[must_use]
    pub fn date_range<T: ColumnTrait>(self, col: T) -> date_range::DateRangeBuilder<T> {
        date_range::DateRangeBuilder::new(self, col)
    }

    #[must_use]
    pub fn build(&self) -> Condition {
        self.condition.clone().into_condition()
    }
}
