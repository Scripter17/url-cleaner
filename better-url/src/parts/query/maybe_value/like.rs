//! [`MaybeQueryLikeValue`].

use crate::prelude::*;

/// Either [`MaybeSpecialQueryValue`] or [`MaybeNonSpecialQueryValue`].
#[derive(Debug, Clone)]
pub enum MaybeQueryLikeValue<'a> {
    /** [`MaybeQueryValue`].         **/ Query   (MaybeQueryValue        <'a>),
    /** [`MaybeFragmentQueryValue`]. **/ Fragment(MaybeFragmentQueryValue<'a>),
}

impl<'a> MaybeQueryLikeValue<'a> {
    /// Either [`Self::new_query`] or [`Self::new_fragment`].
    pub fn new<T: Into<MaybeSpecialQueryValue<'a>> + Into<MaybeNonSpecialQueryValue<'a>> + Into<MaybeFragmentQueryValue<'a>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => Self::new_query   (value, r#type),
            QueryLikeType::Fragment         => Self::new_fragment(value        ),
        }
    }

    /// [`MaybeQueryValue::new`].
    pub fn new_query<T: Into<MaybeSpecialQueryValue<'a>> + Into<MaybeNonSpecialQueryValue<'a>>>(value: T, r#type: QueryType) -> Self {
        MaybeQueryValue::new(value, r#type).into()
    }

    /** [`MaybeSpecialQueryValue::new`].    **/ pub fn new_special    <T: Into<MaybeSpecialQueryValue   <'a>>>(value: T) -> Self {value.into().into()}
    /** [`MaybeNonSpecialQueryValue::new`]. **/ pub fn new_non_special<T: Into<MaybeNonSpecialQueryValue<'a>>>(value: T) -> Self {value.into().into()}
    /** [`MaybeFragmentQueryValue::new`].   **/ pub fn new_fragment   <T: Into<MaybeFragmentQueryValue  <'a>>>(value: T) -> Self {value.into().into()}



    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    /// # Safety
    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => unsafe {Self::new_query_unchecked   (value, r#type)},
            QueryLikeType::Fragment         => unsafe {Self::new_fragment_unchecked(value        )},
        }
    }

    /// [`MaybeQueryValue::new_unchecked`].
    /// # Safety
    /// [`MaybeQueryValue::new_unchecked`].
    pub unsafe fn new_query_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>, r#type: QueryType) -> Self {
        unsafe {MaybeQueryValue::new_unchecked(value, r#type)}.into()
    }

    /// [`MaybeSpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`MaybeSpecialQueryValue::new_unchecked`].
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeSpecialQueryValue::new_unchecked(value)}.into()}

    /// [`MaybeNonSpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`MaybeNonSpecialQueryValue::new_unchecked`].
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeNonSpecialQueryValue::new_unchecked(value)}.into()}

    /// [`MaybeFragmentQueryValue::new_unchecked`].
    /// # Safety
    /// [`MaybeFragmentQueryValue::new_unchecked`].
    pub unsafe fn new_fragment_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeFragmentQueryValue::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }

    /// Either [`MaybeQueryValue::decode`] or [`MaybeFragmentQueryValue::decode`].
    pub fn decode(self) -> Option<Cow<'a, [u8]>> {
        match self {
            Self::Query   (x) => x.decode(),
            Self::Fragment(x) => x.decode(),
        }
    }

    /// Either [`MaybeQueryValue::try_decode`] or [`MaybeFragmentQueryValue::try_decode`].
    /// # Errors
    /// If [`MaybeQueryValue::try_decode`] returns an error, that error is returned.
    ///
    /// If [`MaybeFragmentQueryValue::try_decode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fiiine.")]
    pub fn try_decode(self) -> Option<Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)>> {
        match self {
            Self::Query   (x) => x.try_decode(),
            Self::Fragment(x) => x.try_decode(),
        }
    }

    /// Either [`MaybeQueryValue::lossy_decode`] or [`MaybeFragmentQueryValue::lossy_decode`].
    pub fn lossy_decode(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Query   (x) => x.lossy_decode(),
            Self::Fragment(x) => x.lossy_decode(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeQueryLikeValue<'_> {
        match self {
            Self::Query   (x) => x.borrowed().into(),
            Self::Fragment(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeQueryLikeValue<'static> {
        match self {
            Self::Query   (x) => x.into_owned().into(),
            Self::Fragment(x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Query   (x) => x.into_inner(),
            Self::Fragment(x) => x.into_inner(),
        }
    }

}

impl<'a> From<MaybeQueryValue          <'a>> for MaybeQueryLikeValue<'a> {fn from(value: MaybeQueryValue          <'a>) -> Self {Self::Query   (value       )}}
impl<'a> From<MaybeFragmentQueryValue  <'a>> for MaybeQueryLikeValue<'a> {fn from(value: MaybeFragmentQueryValue  <'a>) -> Self {Self::Fragment(value       )}}
impl<'a> From<MaybeSpecialQueryValue   <'a>> for MaybeQueryLikeValue<'a> {fn from(value: MaybeSpecialQueryValue   <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<MaybeNonSpecialQueryValue<'a>> for MaybeQueryLikeValue<'a> {fn from(value: MaybeNonSpecialQueryValue<'a>) -> Self {Self::Query   (value.into())}}

impl<'a> From<QueryLikeValue<'a>> for MaybeQueryLikeValue<'a> {
    fn from(value: QueryLikeValue<'a>) -> Self {
        match value {
            QueryLikeValue::Query   (x) => x.into(),
            QueryLikeValue::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<QueryValue<'a>> for MaybeQueryLikeValue<'a> {
    fn from(value: QueryValue<'a>) -> Self {
        match value {
            QueryValue::Special   (x) => x.into(),
            QueryValue::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<Option<FragmentQueryValue  <'a>>> for MaybeQueryLikeValue<'a> {fn from(value: Option<FragmentQueryValue  <'a>>) -> Self {Self::Fragment(value.into())}}
impl<'a> From<Option<SpecialQueryValue   <'a>>> for MaybeQueryLikeValue<'a> {fn from(value: Option<SpecialQueryValue   <'a>>) -> Self {Self::Query   (value.into())}}
impl<'a> From<Option<NonSpecialQueryValue<'a>>> for MaybeQueryLikeValue<'a> {fn from(value: Option<NonSpecialQueryValue<'a>>) -> Self {Self::Query   (value.into())}}
