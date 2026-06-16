//! [`QuerySegment`].

use crate::prelude::*;

mod name;
mod value;

/// Either [`SpecialQuerySegment`], [`NonSpecialQuerySegment`], or [`FragmentQuerySegment`].
#[derive(Debug, Clone)]
pub enum QuerySegment<'a> {
    /// [`SpecialQuerySegment`].
    Special(SpecialQuerySegment<'a>),
    /// [`NonSpecialQuerySegment`].
    NonSpecial(NonSpecialQuerySegment<'a>),
    /// [`FragmentQuerySegment`].
    Fragment(FragmentQuerySegment<'a>),
}

impl<'a> QuerySegment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
            Self::Fragment  (x) => x.as_str(),
        }
    }

    /// [`Self::Special`].
    pub fn new_special<T: Into<SpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// [`Self::NonSpec`].
    pub fn new_non_special<T: Into<NonSpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// [`Self::Fragmen`].
    pub fn new_fragment<T: Into<FragmentQuerySegment<'a>>>(value: T) -> Self {
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

    /// If it's [`Self::Fragment`].
    pub fn is_fragment(&self) -> bool {
        matches!(self, Self::Fragment(_))
    }



    /// The [`SpecialQuerySegment`].
    pub fn special(self) -> Option<SpecialQuerySegment<'a>> {
        match self {
            Self::Special   (x) => Some(x),
            Self::NonSpecial(_) => None,
            Self::Fragment  (_) => None,
        }
    }

    /// The [`NonSpecialQuerySegment`].
    pub fn non_special(self) -> Option<NonSpecialQuerySegment<'a>> {
        match self {
            Self::Special   (_) => None,
            Self::NonSpecial(x) => Some(x),
            Self::Fragment  (_) => None,
        }
    }

    /// The [`FragmentQuerySegment`].
    pub fn fragment(self) -> Option<FragmentQuerySegment<'a>> {
        match self {
            Self::Special   (_) => None,
            Self::NonSpecial(_) => None,
            Self::Fragment  (x) => Some(x),
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

    /// Turn into a [`FragmentQuerySegment`]
    pub fn into_fragment(self) -> FragmentQuerySegment<'a> {
        self.into()
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
            Self::Fragment  (x) => x.into_inner(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> QuerySegment<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
            Self::Fragment  (x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> QuerySegment<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
            Self::Fragment  (x) => x.borrowed().into(),
        }
    }
}

impl<'a> From<SpecialQuerySegment   <'a>> for QuerySegment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<NonSpecialQuerySegment<'a>> for QuerySegment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::NonSpecial(value)}}
impl<'a> From<FragmentQuerySegment  <'a>> for QuerySegment<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self::Fragment  (value)}}
