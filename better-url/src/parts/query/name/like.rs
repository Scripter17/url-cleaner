//! [`QueryLikeName`].

use crate::prelude::*;

/// Either [`SpecialQueryName`] or [`NonSpecialQueryName`].
#[derive(Debug, Clone)]
pub enum QueryLikeName<'a> {
    /** [`QueryName`].         **/ Query   (QueryName        <'a>),
    /** [`FragmentQueryName`]. **/ Fragment(FragmentQueryName<'a>),
}

impl<'a> QueryLikeName<'a> {
    /// Either [`Self::new_query`] or [`Self::new_fragment`].
    pub fn new<T: Into<SpecialQueryName<'a>> + Into<NonSpecialQueryName<'a>> + Into<FragmentQueryName<'a>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => Self::new_query   (value, r#type),
            QueryLikeType::Fragment         => Self::new_fragment(value        ),
        }
    }

    /// [`QueryName::new`].
    pub fn new_query<T: Into<SpecialQueryName<'a>> + Into<NonSpecialQueryName<'a>>>(value: T, r#type: QueryType) -> Self {
        QueryName::new(value, r#type).into()
    }

    /** [`SpecialQueryName::new`].    **/ pub fn new_special    <T: Into<SpecialQueryName   <'a>>>(value: T) -> Self {value.into().into()}
    /** [`NonSpecialQueryName::new`]. **/ pub fn new_non_special<T: Into<NonSpecialQueryName<'a>>>(value: T) -> Self {value.into().into()}
    /** [`FragmentQueryName::new`].   **/ pub fn new_fragment   <T: Into<FragmentQueryName  <'a>>>(value: T) -> Self {value.into().into()}



    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    /// # Safety
    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => unsafe {Self::new_query_unchecked   (value, r#type)},
            QueryLikeType::Fragment         => unsafe {Self::new_fragment_unchecked(value        )},
        }
    }

    /// [`QueryName::new_unchecked`].
    /// # Safety
    /// [`QueryName::new_unchecked`].
    pub unsafe fn new_query_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryType) -> Self {
        unsafe {QueryName::new_unchecked(value, r#type)}.into()
    }

    /// [`SpecialQueryName::new_unchecked`].
    /// # Safety
    /// [`SpecialQueryName::new_unchecked`].
    pub unsafe fn new_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {SpecialQueryName::new_unchecked(value)}.into()}

    /// [`NonSpecialQueryName::new_unchecked`].
    /// # Safety
    /// [`NonSpecialQueryName::new_unchecked`].
    pub unsafe fn new_non_special_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {NonSpecialQueryName::new_unchecked(value)}.into()}

    /// [`FragmentQueryName::new_unchecked`].
    /// # Safety
    /// [`FragmentQueryName::new_unchecked`].
    pub unsafe fn new_fragment_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {unsafe {FragmentQueryName::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }

    /// Either [`QueryName::decode`] or [`FragmentQueryName::decode`].
    pub fn decode(self) -> Cow<'a, [u8]> {
        match self {
            Self::Query   (x) => x.decode(),
            Self::Fragment(x) => x.decode(),
        }
    }

    /// Either [`QueryName::try_decode`] or [`FragmentQueryName::try_decode`].
    /// # Errors
    /// If [`QueryName::try_decode`] returns an error, that error is returned.
    ///
    /// If [`FragmentQueryName::try_decode`] returns an error, that error is returned.
    pub fn try_decode(self) -> Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)> {
        match self {
            Self::Query   (x) => x.try_decode(),
            Self::Fragment(x) => x.try_decode(),
        }
    }

    /// Either [`QueryName::lossy_decode`] or [`FragmentQueryName::lossy_decode`].
    pub fn lossy_decode(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.lossy_decode(),
            Self::Fragment(x) => x.lossy_decode(),
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> QueryLikeName<'_> {
        match self {
            Self::Query   (x) => x.borrowed().into(),
            Self::Fragment(x) => x.borrowed().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> QueryLikeName<'static> {
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

impl<'a> From<QueryName          <'a>> for QueryLikeName<'a> {fn from(value: QueryName          <'a>) -> Self {Self::Query   (value       )}}
impl<'a> From<FragmentQueryName  <'a>> for QueryLikeName<'a> {fn from(value: FragmentQueryName  <'a>) -> Self {Self::Fragment(value       )}}
impl<'a> From<SpecialQueryName   <'a>> for QueryLikeName<'a> {fn from(value: SpecialQueryName   <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQueryName<'a>> for QueryLikeName<'a> {fn from(value: NonSpecialQueryName<'a>) -> Self {Self::Query   (value.into())}}
