//! [`QueryName`].

use crate::prelude::*;

/// Either [`SpecialQueryName`] or [`NonSpecialQueryName`].
#[derive(Debug, Clone)]
pub enum QueryName<'a> {
    /** [`SpecialQueryName`].    **/ Special   (SpecialQueryName   <'a>),
    /** [`NonSpecialQueryName`]. **/ NonSpecial(NonSpecialQueryName<'a>),
}

impl<'a> QueryName<'a> {
    /// Either [`Self::new_special`] or [`Self::new_non_special`].
    pub fn new<T: Into<SpecialQueryName<'a>> + Into<NonSpecialQueryName<'a>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => Self::new_special    (value),
            QueryType::NonSpecial => Self::new_non_special(value),
        }
    }

    /** [`SpecialQueryName::new`].    **/ pub fn new_special    <T: Into<SpecialQueryName   <'a>>>(value: T) -> Self {value.into().into()}
    /** [`NonSpecialQueryName::new`]. **/ pub fn new_non_special<T: Into<NonSpecialQueryName<'a>>>(value: T) -> Self {value.into().into()}



    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    /// # Safety
    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => unsafe {Self::new_special_unchecked    (value)},
            QueryType::NonSpecial => unsafe {Self::new_non_special_unchecked(value)},
        }
    }

    /// [`SpecialQueryName::new_unchecked`].
    /// # Safety
    /// [`SpecialQueryName::new_unchecked`].
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialQueryName::new_unchecked(value)}.into()}

    /// [`NonSpecialQueryName::new_unchecked`].
    /// # Safety
    /// [`NonSpecialQueryName::new_unchecked`].
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialQueryName::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
        }
    }

    /// Either [`SpecialQueryName::decode`] or [`NonSpecialQueryName::decode`].
    pub fn decode(self) -> Cow<'a, [u8]> {
        match self {
            Self::Special   (x) => x.decode(),
            Self::NonSpecial(x) => x.decode(),
        }
    }

    /// Either [`SpecialQueryName::try_decode`] or [`NonSpecialQueryName::try_decode`].
    /// # Errors
    /// If [`SpecialQueryName::try_decode`] returns an error, that error is returned.
    ///
    /// If [`NonSpecialQueryName::try_decode`] returns an error, that error is returned.
    pub fn try_decode(self) -> Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)> {
        match self {
            Self::Special   (x) => x.try_decode(),
            Self::NonSpecial(x) => x.try_decode(),
        }
    }

    /// Either [`SpecialQueryName::lossy_decode`] or [`NonSpecialQueryName::lossy_decode`].
    pub fn lossy_decode(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.lossy_decode(),
            Self::NonSpecial(x) => x.lossy_decode(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> QueryName<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> QueryName<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
        }
    }

}

impl<'a> From<SpecialQueryName   <'a>> for QueryName<'a> {fn from(value: SpecialQueryName   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<NonSpecialQueryName<'a>> for QueryName<'a> {fn from(value: NonSpecialQueryName<'a>) -> Self {Self::NonSpecial(value)}}
