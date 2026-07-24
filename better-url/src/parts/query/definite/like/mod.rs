//! [`QueryLike`].

use crate::prelude::*;

mod get;
mod set;

/// Either [`Query`] or [`FragmentQuery`].
#[derive(Debug, Clone)]
pub enum QueryLike<'a> {
    /** [`Query`].         **/ Query   (Query        <'a>),
    /** [`FragmentQuery`]. **/ Fragment(FragmentQuery<'a>),
}

impl<'a> QueryLike<'a> {
    /// Either [`Query::as_str`] or [`FragmentQuery::as_str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }

    /// [`Self::len`] + 1 for the `?`.
    pub fn search_len(&self) -> usize {
        self.len() + 1
    }

    /// The [`QueryLikeType`].
    pub fn r#type(&self) -> QueryLikeType {
        match self {
            Self::Query   (x) => x.r#type().into()      ,
            Self::Fragment(_) => QueryLikeType::Fragment,
        }
    }



    /// Make a new [`Self`] of the [`QueryLikeType`].
    pub fn new<T: Into<SpecialQuery<'a>> + Into<NonSpecialQuery<'a>> + Into<FragmentQuery<'a>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => Self::new_query   (value, r#type),
            QueryLikeType::Fragment         => Self::new_fragment(value        ),
        }
    }

    /** [`Query::new`].           **/ pub fn new_query            <T: Into<SpecialQuery   <'a>> + Into<NonSpecialQuery<'a>>>(value: T, r#type: QueryType) -> Self {Query          ::new(value, r#type).into()}
    /** [`SpecialQuery::new`].    **/ pub fn new_special_query    <T: Into<SpecialQuery   <'a>>                            >(value: T                   ) -> Self {SpecialQuery   ::new(value        ).into()}
    /** [`NonSpecialQuery::new`]. **/ pub fn new_non_special_query<T: Into<NonSpecialQuery<'a>>                            >(value: T                   ) -> Self {NonSpecialQuery::new(value        ).into()}
    /** [`FragmentQuery::new`].   **/ pub fn new_fragment         <T: Into<FragmentQuery  <'a>>                            >(value: T                   ) -> Self {FragmentQuery  ::new(value        ).into()}



    /// Make a new [`Self`] of the [`QueryLikeType`] without validity checks.
    /// # Safety
    /// If `type` is [`QueryLikeType::Query`], it and `value` must be valid inputs to [`Self::new_query_unchecked`].
    ///
    /// If `type` is [`QueryLikeType::Fragment`], value must be a valid input to [`Self::new_fragment_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => unsafe {Self::new_query_unchecked   (value, r#type)},
            QueryLikeType::Fragment         => unsafe {Self::new_fragment_unchecked(value        )},
        }
    }

    /// [`Query::new_unchecked`].
    /// # Safety
    /// [`Query::new_unchecked`].
    pub unsafe fn new_query_unchecked            <T: Into<Cow<'a, str>>>(value: T, r#type: QueryType) -> Self {unsafe {Query          ::new_unchecked(value, r#type)}.into()}

    /// [`SpecialQuery::new_unchecked`].
    /// # Safety
    /// [`SpecialQuery::new_unchecked`].
    pub unsafe fn new_special_query_unchecked    <T: Into<Cow<'a, str>>>(value: T                   ) -> Self {unsafe {SpecialQuery   ::new_unchecked(value        )}.into()}

    /// [`NonSpecialQuery::new_unchecked`].
    /// # Safety
    /// [`NonSpecialQuery::new_unchecked`].
    pub unsafe fn new_non_special_query_unchecked<T: Into<Cow<'a, str>>>(value: T                   ) -> Self {unsafe {NonSpecialQuery::new_unchecked(value        )}.into()}

    /// [`FragmentQuery::new_unchecked`].
    /// # Safety
    /// [`FragmentQuery::new_unchecked`].
    pub unsafe fn new_fragment_unchecked         <T: Into<Cow<'a, str>>>(value: T                   ) -> Self {unsafe {FragmentQuery  ::new_unchecked(value        )}.into()}



    /// Either [`Query::borrowed`] or [`FragmentQuery::borrowed`].
    pub fn borrowed(&self) -> QueryLike<'_> {
        match self {
            Self::Query   (x) => x.borrowed().into(),
            Self::Fragment(x) => x.borrowed().into(),
        }
    }

    /// Either [`Query::into_owned`] or [`FragmentQuery::into_owned`].
    pub fn into_owned(self) -> QueryLike<'static> {
        match self {
            Self::Query   (x) => x.into_owned().into(),
            Self::Fragment(x) => x.into_owned().into(),
        }
    }

    /// Either [`Query::into_inner`] or [`FragmentQuery::into_inner`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.into_inner(),
            Self::Fragment(x) => x.into_inner(),
        }
    }
}

impl<'a> From<Query          <'a>> for QueryLike<'a> {fn from(value: Query          <'a>) -> Self {Self::Query   (value       )}}
impl<'a> From<SpecialQuery   <'a>> for QueryLike<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQuery<'a>> for QueryLike<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<FragmentQuery  <'a>> for QueryLike<'a> {fn from(value: FragmentQuery  <'a>) -> Self {Self::Fragment(value       )}}
impl<'a> From<Fragment       <'a>> for QueryLike<'a> {fn from(value: Fragment       <'a>) -> Self {Self::Fragment(value.into())}}

impl<'a> From<QuerySegment          <'a>> for QueryLike<'a> {fn from(value: QuerySegment          <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<SpecialQuerySegment   <'a>> for QueryLike<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQuerySegment<'a>> for QueryLike<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self::Query   (value.into())}}
impl<'a> From<FragmentQuerySegment  <'a>> for QueryLike<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self::Fragment(value.into())}}
