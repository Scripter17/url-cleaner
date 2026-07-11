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



    /// Make a new [`Self`].
    pub fn new<T: Into<SpecialQuerySegment<'a>> + Into<NonSpecialQuerySegment<'a>>>(value: T, special: bool) -> Self {
        match special {
            true  => Self::new_special    (value),
            false => Self::new_non_special(value),
        }
    }

    /// Make a new [`Self::Special`].
    pub fn new_special<T: Into<SpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }

    /// Make a new [`Self::NonSpecial`].
    pub fn new_non_special<T: Into<NonSpecialQuerySegment<'a>>>(value: T) -> Self {
        value.into().into()
    }



    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, special: bool) -> Self {
        unsafe {
            match special {
                true  => Self::new_special_unchecked    (value),
                false => Self::new_non_special_unchecked(value),
            }
        }
    }

    /// Make a new [`Self::Special`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self::Special`] literal.
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        unsafe {
            SpecialQuerySegment::new_unchecked(value).into()
        }
    }

    /// Make a new [`Self::NonSpecial`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self::NonSpecial`] literal.
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        unsafe {
            NonSpecialQuerySegment::new_unchecked(value).into()
        }
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
