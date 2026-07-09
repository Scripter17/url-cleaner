//! [`QuerySegment`].

use crate::prelude::*;

mod name;
mod value;

/// Either [`SpecialQuerySegment`] or [`NonSpecialQuerySegment`].
#[derive(Debug, Clone)]
pub enum QuerySegment<'a> {
    /// [`SpecialQuerySegment`].
    Special(SpecialQuerySegment<'a>),
    /// [`NonSpecialQuerySegment`].
    NonSpecial(NonSpecialQuerySegment<'a>),
}

impl<'a> QuerySegment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
        }
    }

    /// [`Self::Special`].
    pub fn new_special<T: Into<SpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// [`Self::NonSpecial`].
    pub fn new_non_special<T: Into<NonSpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }



    /// If it's [`Self::Special`].
    pub fn is_special(&self) -> bool {
        matches!(self, Self::Special(_))
    }

    /// If it's [`Self::NonSpecial`].
    pub fn is_non_special(&self) -> bool {
        matches!(self, Self::NonSpecial(_))
    }



    /// Either [`SpecialQuerySegment::borrowed`] or [`NonSpecialQuerySegment::borrowed`].
    pub fn borrowed(&self) -> QuerySegment<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
        }
    }

    /// Either [`SpecialQuerySegment::into_owned`] or [`NonSpecialQuerySegment::into_owned`].
    pub fn into_owned(self) -> QuerySegment<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
        }
    }

    /// Either [`SpecialQuerySegment::into_inner`] or [`NonSpecialQuerySegment::into_inner`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
        }
    }
}

impl<'a> TryFrom<QueryLikeSegment<'a>> for QuerySegment<'a> {
    type Error = FragmentQuerySegment<'a>;

    fn try_from(value: QueryLikeSegment<'a>) -> Result<Self, Self::Error> {
        match value {
            QueryLikeSegment::Query   (x) => Ok (x),
            QueryLikeSegment::Fragment(x) => Err(x),
        }
    }
}

impl<'a> From<SpecialQuerySegment   <'a>> for QuerySegment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<NonSpecialQuerySegment<'a>> for QuerySegment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::NonSpecial(value)}}
