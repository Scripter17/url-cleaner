//! [`MaybeQueryValue`].

use crate::prelude::*;

/// Either [`MaybeSpecialQueryValue`] or [`MaybeNonSpecialQueryValue`].
#[derive(Debug, Clone)]
pub enum MaybeQueryValue<'a> {
    /** [`MaybeSpecialQueryValue`].    **/ Special   (MaybeSpecialQueryValue   <'a>),
    /** [`MaybeNonSpecialQueryValue`]. **/ NonSpecial(MaybeNonSpecialQueryValue<'a>),
}

impl<'a> MaybeQueryValue<'a> {
    /// Either [`Self::new_special`] or [`Self::new_non_special`].
    pub fn new<T: Into<MaybeSpecialQueryValue<'a>> + Into<MaybeNonSpecialQueryValue<'a>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => Self::new_special    (value),
            QueryType::NonSpecial => Self::new_non_special(value),
        }
    }

    /** [`MaybeSpecialQueryValue::new`].    **/ pub fn new_special    <T: Into<MaybeSpecialQueryValue   <'a>>>(value: T) -> Self {value.into().into()}
    /** [`MaybeNonSpecialQueryValue::new`]. **/ pub fn new_non_special<T: Into<MaybeNonSpecialQueryValue<'a>>>(value: T) -> Self {value.into().into()}



    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    /// # Safety
    /// Either [`Self::new_special_unchecked`] or [`Self::new_non_special_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => unsafe {Self::new_special_unchecked    (value)},
            QueryType::NonSpecial => unsafe {Self::new_non_special_unchecked(value)},
        }
    }

    /// [`MaybeSpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`MaybeSpecialQueryValue::new_unchecked`].
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeSpecialQueryValue::new_unchecked(value)}.into()}

    /// [`MaybeNonSpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`MaybeNonSpecialQueryValue::new_unchecked`].
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeNonSpecialQueryValue::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Special   (x) => x.as_str(),
            Self::NonSpecial(x) => x.as_str(),
        }
    }

    /// Either [`MaybeSpecialQueryValue::decode`] or [`MaybeNonSpecialQueryValue::decode`].
    pub fn decode(self) -> Option<Cow<'a, [u8]>> {
        match self {
            Self::Special   (x) => x.decode(),
            Self::NonSpecial(x) => x.decode(),
        }
    }

    /// Either [`MaybeSpecialQueryValue::try_decode`] or [`MaybeNonSpecialQueryValue::try_decode`].
    /// # Errors
    /// If [`MaybeSpecialQueryValue::try_decode`] returns an error, that error is returned.
    ///
    /// If [`MaybeNonSpecialQueryValue::try_decode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fiiine.")]
    pub fn try_decode(self) -> Option<Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)>> {
        match self {
            Self::Special   (x) => x.try_decode(),
            Self::NonSpecial(x) => x.try_decode(),
        }
    }

    /// Either [`MaybeSpecialQueryValue::lossy_decode`] or [`MaybeNonSpecialQueryValue::lossy_decode`].
    pub fn lossy_decode(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Special   (x) => x.lossy_decode(),
            Self::NonSpecial(x) => x.lossy_decode(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeQueryValue<'_> {
        match self {
            Self::Special   (x) => x.borrowed().into(),
            Self::NonSpecial(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeQueryValue<'static> {
        match self {
            Self::Special   (x) => x.into_owned().into(),
            Self::NonSpecial(x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Special   (x) => x.into_inner(),
            Self::NonSpecial(x) => x.into_inner(),
        }
    }

}

impl<'a> From<MaybeSpecialQueryValue   <'a>> for MaybeQueryValue<'a> {fn from(value: MaybeSpecialQueryValue   <'a>) -> Self {Self::Special   (value)}}
impl<'a> From<MaybeNonSpecialQueryValue<'a>> for MaybeQueryValue<'a> {fn from(value: MaybeNonSpecialQueryValue<'a>) -> Self {Self::NonSpecial(value)}}

impl<'a> From<QueryValue<'a>> for MaybeQueryValue<'a> {
    fn from(value: QueryValue<'a>) -> Self {
        match value {
            QueryValue::Special   (x) => x.into(),
            QueryValue::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<Option<SpecialQueryValue   <'a>>> for MaybeQueryValue<'a> {fn from(value: Option<SpecialQueryValue   <'a>>) -> Self {Self::Special   (value.into())}}
impl<'a> From<Option<NonSpecialQueryValue<'a>>> for MaybeQueryValue<'a> {fn from(value: Option<NonSpecialQueryValue<'a>>) -> Self {Self::NonSpecial(value.into())}}
