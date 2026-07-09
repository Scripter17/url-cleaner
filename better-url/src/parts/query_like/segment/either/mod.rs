//! [`QueryLikeSegment`].

use crate::prelude::*;

mod name;
mod value;

/// Either [`QuerySegment`] or [`FragmentQuerySegment`].
#[derive(Debug, Clone)]
pub enum QueryLikeSegment<'a> {
    /// [`QuerySegment`].
    Query(QuerySegment<'a>),
    /// [`FragmentQuerySegment`].
    Fragment(FragmentQuerySegment<'a>),
}

impl<'a> QueryLikeSegment<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(value: T) -> Self {
        value.into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }



    /// Make a new [`Self`] from a [`QuerySegment`].
    pub fn new_query<T: Into<QuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// Make a new [`Self`] from a [`SpecialQuerySegment`].
    pub fn new_special_query<T: Into<SpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// Make a new [`Self`] from a [`NonSpecialQuerySegment`].
    pub fn new_non_special_query<T: Into<NonSpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// Make a new [`Self`] from a [`FragmentQuerySegment`].
    pub fn new_fragment<T: Into<FragmentQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }



    /// If it's [`Self::Query`].
    pub fn is_query(&self) -> bool {
        matches!(self, Self::Query(_))
    }

    /// If it's [`Self::Query`] and [`Query::Special`].
    pub fn is_special_query(&self) -> bool {
        matches!(self, Self::Query(QuerySegment::Special(_)))
    }

    /// If it's [`Self::Query`] and [`Query::NonSpecial`].
    pub fn is_non_special_query(&self) -> bool {
        matches!(self, Self::Query(QuerySegment::NonSpecial(_)))
    }

    /// If it's [`Self::Fragment`].
    pub fn is_fragment(&self) -> bool {
        matches!(self, Self::Fragment(_))
    }




    /// Either [`QuerySegment::borrowed`] or [`FragmentQuerySegment::borrowed`].
    pub fn borrowed(&self) -> QueryLikeSegment<'_> {
        match self {
            Self::Query   (x) => x.borrowed().into(),
            Self::Fragment(x) => x.borrowed().into(),
        }
    }

    /// Either [`QuerySegment::into_owned`] or [`FragmentQuerySegment::into_owned`].
    pub fn into_owned(self) -> QueryLikeSegment<'static> {
        match self {
            Self::Query   (x) => x.into_owned().into(),
            Self::Fragment(x) => x.into_owned().into(),
        }
    }

    /// Either [`QuerySegment::into_inner`] or [`FragmentQuerySegment::into_inner`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.into_inner(),
            Self::Fragment(x) => x.into_inner(),
        }
    }
}

impl<'a> From<QuerySegment          <'a>> for QueryLikeSegment<'a> {fn from(value: QuerySegment          <'a>) -> Self {Self::Query   (value       )}}
impl<'a> From<SpecialQuerySegment   <'a>> for QueryLikeSegment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQuerySegment<'a>> for QueryLikeSegment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<FragmentQuerySegment  <'a>> for QueryLikeSegment<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self::Fragment(value       )}}
