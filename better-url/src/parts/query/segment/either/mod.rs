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

    /// Make a new [`Self`] from a pair.
    pub fn from_pair<T: Into<Cow<'a, str>>>(name: T, value: Option<&str>) -> Self {
        SpecialQuerySegment::from_pair(name, value).into()
    }



    /// If it's [`Self::Special`].
    pub fn is_special(&self) -> bool {
        matches!(self, Self::Special(_))
    }

    /// If it's [`Self::NonSpecial`].
    pub fn is_non_special(&self) -> bool {
        matches!(self, Self::NonSpecial(_))
    }



    /// The [`SpecialQuerySegment`].
    pub fn special(self) -> Option<SpecialQuerySegment<'a>> {
        match self {
            Self::Special   (x) => Some(x),
            Self::NonSpecial(_) => None
        }
    }

    /// The [`NonSpecialQuerySegment`].
    pub fn non_special(self) -> Option<NonSpecialQuerySegment<'a>> {
        match self {
            Self::Special   (_) => None,
            Self::NonSpecial(x) => Some(x),
        }
    }



    /// Turn into a [`SpecialQuerySegment`].
    pub fn into_special(self) -> SpecialQuerySegment<'a> {
        self.into()
    }

    /// Turn into a [`NonSpecialQuerySegment`].
    pub fn into_non_special(self) -> NonSpecialQuerySegment<'a> {
        self.into()
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> QuerySegment<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> QuerySegment<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
        }
    }
}

impl<'a> From<Cow<'a, str>> for QuerySegment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self::Special(value.into())
    }
}

impl<'a> From<SpecialQuerySegment   <'a>> for QuerySegment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<NonSpecialQuerySegment<'a>> for QuerySegment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::NonSpecial(value)}}
