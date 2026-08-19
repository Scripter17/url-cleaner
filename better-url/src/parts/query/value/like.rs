//! [`QueryLikeValue`].

use crate::prelude::*;

/// Either [`SpecialQueryValue`] or [`NonSpecialQueryValue`].
#[derive(Debug, Clone)]
pub enum QueryLikeValue<'a> {
    /** [`QueryValue`].         **/ Query   (QueryValue        <'a>),
    /** [`FragmentQueryValue`]. **/ Fragment(FragmentQueryValue<'a>),
}

impl<'a> QueryLikeValue<'a> {
    /// Either [`Self::new_query`] or [`Self::new_fragment`].
    pub fn new<T: Into<SpecialQueryValue<'a>> + Into<NonSpecialQueryValue<'a>> + Into<FragmentQueryValue<'a>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => Self::new_query   (value, r#type),
            QueryLikeType::Fragment         => Self::new_fragment(value        ),
        }
    }

    /// [`QueryValue::new`].
    pub fn new_query<T: Into<SpecialQueryValue<'a>> + Into<NonSpecialQueryValue<'a>>>(value: T, r#type: QueryType) -> Self {
        QueryValue::new(value, r#type).into()
    }

    /** [`SpecialQueryValue::new`].    **/ pub fn new_special    <T: Into<SpecialQueryValue   <'a>>>(value: T) -> Self {value.into().into()}
    /** [`NonSpecialQueryValue::new`]. **/ pub fn new_non_special<T: Into<NonSpecialQueryValue<'a>>>(value: T) -> Self {value.into().into()}
    /** [`FragmentQueryValue::new`].   **/ pub fn new_fragment   <T: Into<FragmentQueryValue  <'a>>>(value: T) -> Self {value.into().into()}



    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    /// # Safety
    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => unsafe {Self::new_query_unchecked   (value, r#type)},
            QueryLikeType::Fragment         => unsafe {Self::new_fragment_unchecked(value        )},
        }
    }

    /// [`QueryValue::new_unchecked`].
    /// # Safety
    /// [`QueryValue::new_unchecked`].
    pub unsafe fn new_query_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryType) -> Self {
        unsafe {QueryValue::new_unchecked(value, r#type)}.into()
    }

    /// [`SpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`SpecialQueryValue::new_unchecked`].
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialQueryValue::new_unchecked(value)}.into()}

    /// [`NonSpecialQueryValue::new_unchecked`].
    /// # Safety
    /// [`NonSpecialQueryValue::new_unchecked`].
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialQueryValue::new_unchecked(value)}.into()}

    /// [`FragmentQueryValue::new_unchecked`].
    /// # Safety
    /// [`FragmentQueryValue::new_unchecked`].
    pub unsafe fn new_fragment_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {FragmentQueryValue::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }

    /// Either [`QueryValue::decode`] or [`FragmentQueryValue::decode`].
    pub fn decode(self) -> Cow<'a, [u8]> {
        match self {
            Self::Query   (x) => x.decode(),
            Self::Fragment(x) => x.decode(),
        }
    }

    /// Either [`QueryValue::try_decode`] or [`FragmentQueryValue::try_decode`].
    /// # Errors
    /// If [`QueryValue::try_decode`] returns an error, that error is returned.
    ///
    /// If [`FragmentQueryValue::try_decode`] returns an error, that error is returned.
    pub fn try_decode(self) -> Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)> {
        match self {
            Self::Query   (x) => x.try_decode(),
            Self::Fragment(x) => x.try_decode(),
        }
    }

    /// Either [`QueryValue::lossy_decode`] or [`FragmentQueryValue::lossy_decode`].
    pub fn lossy_decode(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.lossy_decode(),
            Self::Fragment(x) => x.lossy_decode(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> QueryLikeValue<'_> {
        match self {
            Self::Query   (x) => x.borrowed().into(),
            Self::Fragment(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> QueryLikeValue<'static> {
        match self {
            Self::Query   (x) => x.into_owned().into(),
            Self::Fragment(x) => x.into_owned().into(),
        }
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.into_inner(),
            Self::Fragment(x) => x.into_inner(),
        }
    }

}

impl<'a> From<QueryValue          <'a>> for QueryLikeValue<'a> {fn from(value: QueryValue          <'a>) -> Self {Self::Query   (value       )}}
impl<'a> From<FragmentQueryValue  <'a>> for QueryLikeValue<'a> {fn from(value: FragmentQueryValue  <'a>) -> Self {Self::Fragment(value       )}}
impl<'a> From<SpecialQueryValue   <'a>> for QueryLikeValue<'a> {fn from(value: SpecialQueryValue   <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQueryValue<'a>> for QueryLikeValue<'a> {fn from(value: NonSpecialQueryValue<'a>) -> Self {Self::Query   (value.into())}}
