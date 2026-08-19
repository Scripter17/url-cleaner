//! [`QueryValue`].

use crate::prelude::*;

/// Either [`SpecialQueryValue`] or [`NonSpecialQueryValue`].
#[derive(Debug, Clone)]
pub enum QueryValue<'a> {
    /** [`SpecialQueryValue`].    **/ Special   (SpecialQueryValue   <'a>),
    /** [`NonSpecialQueryValue`]. **/ NonSpecial(NonSpecialQueryValue<'a>),
}

impl<'a> QueryValue<'a> {
    /// Either [`Self::new_special`] or [`Self::new_non_special`].
    pub fn new<T: Into<SpecialQueryValue<'a>> + Into<NonSpecialQueryValue<'a>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => Self::new_special    (value),
            QueryType::NonSpecial => Self::new_non_special(value),
        }
    }

    /** [`SpecialQueryValue::new`].    **/ pub fn new_special    <T: Into<SpecialQueryValue   <'a>>>(value: T) -> Self {value.into().into()}
    /** [`NonSpecialQueryValue::new`]. **/ pub fn new_non_special<T: Into<NonSpecialQueryValue<'a>>>(value: T) -> Self {value.into().into()}



    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    /// # Safety
    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => unsafe {Self::new_special_unchecked    (value)},
            QueryType::NonSpecial => unsafe {Self::new_non_special_unchecked(value)},
        }
    }

    /// [`SpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`SpecialQueryValue::new_unchecked`].
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialQueryValue::new_unchecked(value)}.into()}

    /// [`NonSpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`NonSpecialQueryValue::new_unchecked`].
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialQueryValue::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
        }
    }

    /// Either [`SpecialQueryValue::decode`] or [`NonSpecialQueryValue::decode`].
    pub fn decode(self) -> Cow<'a, [u8]> {
        match self {
            Self::Special   (x) => x.decode(),
            Self::NonSpecial(x) => x.decode(),
        }
    }

    /// Either [`SpecialQueryValue::try_decode`] or [`NonSpecialQueryValue::try_decode`].
    /// # Errors
    /// If [`SpecialQueryValue::try_decode`] returns an error, that error is returned.
    ///
    /// If [`NonSpecialQueryValue::try_decode`] returns an error, that error is returned.
    pub fn try_decode(self) -> Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)> {
        match self {
            Self::Special   (x) => x.try_decode(),
            Self::NonSpecial(x) => x.try_decode(),
        }
    }

    /// Either [`SpecialQueryValue::lossy_decode`] or [`NonSpecialQueryValue::lossy_decode`].
    pub fn lossy_decode(self) -> Cow<'a, str> {
        match self {
            Self::Special   (x) => x.lossy_decode(),
            Self::NonSpecial(x) => x.lossy_decode(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> QueryValue<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> QueryValue<'static> {
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

impl<'a> From<SpecialQueryValue   <'a>> for QueryValue<'a> {fn from(value: SpecialQueryValue   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<NonSpecialQueryValue<'a>> for QueryValue<'a> {fn from(value: NonSpecialQueryValue<'a>) -> Self {Self::NonSpecial(value)}}
