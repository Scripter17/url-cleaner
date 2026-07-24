//! [`QueryType`] and [`QueryLikeType`].

use crate::prelude::*;

/// The type of a [`QueryLike`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QueryLikeType {
    /** [`QueryLike::Query`].    **/ Query(QueryType),
    /** [`QueryLike::Fragment`]. **/ Fragment,
}

impl From<QueryType> for QueryLikeType {
    fn from(value: QueryType) -> Self {
        Self::Query(value)
    }
}

/// The type of a [`Query`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QueryType {
    /** [`Query::Special`].    **/ Special   ,
    /** [`Query::NonSpecial`]. **/ NonSpecial,
}

impl From<SchemeType> for QueryType {
    fn from(value: SchemeType) -> Self {
        match value {
            SchemeType::File | SchemeType::SpecialNotFile => Self::Special   ,
            SchemeType::NonSpecial                        => Self::NonSpecial,
        }
    }
}
