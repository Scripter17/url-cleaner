//! Glue to make diesel suck slightly less.

use std::time::Duration;

use diesel::{Queryable, AsExpression};
use diesel::backend::Backend;
use diesel::sql_types::Float;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};

/// Glue to make [`Duration`] work in diesel.
#[derive(Debug, Clone, Copy, AsExpression)]
#[diesel(sql_type = Float)]
pub struct DurationGlue {
    /// The [`Duration`] as a [`Duration`].
    d: Duration,
    /// The [`Duration`] as [`Duration::as_secs_f32`].
    f: f32
}

impl From<DurationGlue> for Duration {
    fn from(value: DurationGlue) -> Self {
        value.d
    }
}

impl From<Duration> for DurationGlue {
    fn from(value: Duration) -> Self {
        Self {
            d: value,
            f: value.as_secs_f32()
        }
    }
}

impl<DB> Queryable<Float, DB> for DurationGlue
where
    DB: Backend,
    f32: FromSql<Float, DB>
{
    type Row = f32;

    fn build(value: f32) -> deserialize::Result<Self> {
        Ok(DurationGlue {
            d: Duration::from_secs_f32(value),
            f: value
        })
    }
}

impl<DB> ToSql<Float, DB> for DurationGlue
    where
        DB: Backend,
        f32: ToSql<Float, DB>
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        self.f.to_sql(out)
    }
}

