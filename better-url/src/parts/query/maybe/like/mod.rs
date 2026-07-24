//! [`MaybeQueryLike`].

use crate::prelude::*;

mod get;
mod set;
mod filter;

/// Either [`MaybeQuery`] or [`MaybeFragmentQuery`].
#[derive(Debug, Clone)]
pub enum MaybeQueryLike<'a> {
    /** [`MaybeQuery`].         **/ Query   (MaybeQuery        <'a>),
    /** [`MaybeFragmentQuery`]. **/ Fragment(MaybeFragmentQuery<'a>),
}

impl<'a> MaybeQueryLike<'a> {
    /// Either [`Self::new_query`] or [`Self::new_fragment`].
    pub fn new<T: Into<MaybeSpecialQuery<'a>> + Into<MaybeNonSpecialQuery<'a>> + Into<MaybeFragmentQuery<'a>>>(value: T, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => Self::new_query   (value, r#type),
            QueryLikeType::Fragment         => Self::new_fragment(value        ),
        }
    }

    /// Either [`Self::new_special_query`] or [`Self::new_non_special_query`].
    pub fn new_query<T: Into<MaybeSpecialQuery<'a>> + Into<MaybeNonSpecialQuery<'a>>>(value: T, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => Self::new_special_query    (value),
            QueryType::NonSpecial => Self::new_non_special_query(value),
        }
    }

    /** [`MaybeSpecialQuery::new`].    **/ pub fn new_special_query    <T: Into<MaybeSpecialQuery   <'a>>>(value: T) -> Self {MaybeSpecialQuery   ::new(value).into()}
    /** [`MaybeNonSpecialQuery::new`]. **/ pub fn new_non_special_query<T: Into<MaybeNonSpecialQuery<'a>>>(value: T) -> Self {MaybeNonSpecialQuery::new(value).into()}
    /** [`MaybeFragmentQuery::new`].   **/ pub fn new_fragment         <T: Into<MaybeFragmentQuery  <'a>>>(value: T) -> Self {MaybeFragmentQuery  ::new(value).into()}



    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    /// # Safety
    /// Either [`Self::new_query_unchecked`] or [`Self::new_fragment_unchecked`].
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>, r#type: QueryLikeType) -> Self {
        match r#type {
            QueryLikeType::Query   (r#type) => unsafe {Self::new_query_unchecked   (value, r#type)},
            QueryLikeType::Fragment         => unsafe {Self::new_fragment_unchecked(value        )},
        }
    }

    /// Either [`Self::new_special_query_unchecked`] or [`Self::new_non_special_query_unchecked`].
    /// # Safety
    /// Either [`Self::new_special_query_unchecked`] or [`Self::new_non_special_query_unchecked`].
    pub unsafe fn new_query_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>, r#type: QueryType) -> Self {
        match r#type {
            QueryType::Special    => unsafe {Self::new_special_query_unchecked    (value)},
            QueryType::NonSpecial => unsafe {Self::new_non_special_query_unchecked(value)},
        }
    }

    /// [`MaybeSpecialQuery::new_unchecked`].
    /// # Safety
    /// [`MaybeSpecialQuery::new_unchecked`].
    pub unsafe fn new_special_query_unchecked    <T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeSpecialQuery   ::new_unchecked(value)}.into()}

    /// [`MaybeNonSpecialQuery::new_unchecked`].
    /// # Safety
    /// [`MaybeNonSpecialQuery::new_unchecked`].
    pub unsafe fn new_non_special_query_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeNonSpecialQuery::new_unchecked(value)}.into()}

    /// [`MaybeFragmentQuery::new_unchecked`].
    /// # Safety
    /// [`MaybeFragmentQuery::new_unchecked`].
    pub unsafe fn new_fragment_unchecked         <T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {unsafe {MaybeFragmentQuery  ::new_unchecked(value)}.into()}



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }



    /// If it's [`Self::Query`].
    pub fn is_query(&self) -> bool {
        matches!(self, Self::Query(_))
    }

    /// If it's [`Self::Query`] and [`MaybeQuery::Special`].
    pub fn is_special_query(&self) -> bool {
        matches!(self, Self::Query(MaybeQuery::Special(_)))
    }

    /// If it's [`Self::Query`] and [`MaybeQuery::NonSpecial`].
    pub fn is_non_special_query(&self) -> bool {
        matches!(self, Self::Query(MaybeQuery::NonSpecial(_)))
    }

    /// If it's [`Self::Fragment`].
    pub fn is_fragment(&self) -> bool {
        matches!(self, Self::Fragment(_))
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeQueryLike<'static> {
        match self {
            Self::Query   (x) => x.into_owned().into(),
            Self::Fragment(x) => x.into_owned().into(),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Query   (x) => x.into_inner(),
            Self::Fragment(x) => x.into_inner(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeQueryLike<'_> {
        match self {
            Self::Query   (x) => x.borrowed().into(),
            Self::Fragment(x) => x.borrowed().into(),
        }
    }
}

impl<'a> From<Query                        <'a>>  for MaybeQueryLike<'a> {fn from(value: Query                        <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<SpecialQuery                 <'a>>  for MaybeQueryLike<'a> {fn from(value: SpecialQuery                 <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQuery              <'a>>  for MaybeQueryLike<'a> {fn from(value: NonSpecialQuery              <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<Fragment                     <'a>>  for MaybeQueryLike<'a> {fn from(value: Fragment                     <'a> ) -> Self {Self::Fragment(value.into())}}
impl<'a> From<FragmentQuery                <'a>>  for MaybeQueryLike<'a> {fn from(value: FragmentQuery                <'a> ) -> Self {Self::Fragment(value.into())}}

impl<'a> From<Option<SpecialQuery          <'a>>> for MaybeQueryLike<'a> {fn from(value: Option<SpecialQuery          <'a>>) -> Self {Self::Query   (value.into())}}
impl<'a> From<Option<NonSpecialQuery       <'a>>> for MaybeQueryLike<'a> {fn from(value: Option<NonSpecialQuery       <'a>>) -> Self {Self::Query   (value.into())}}
impl<'a> From<Option<Fragment              <'a>>> for MaybeQueryLike<'a> {fn from(value: Option<Fragment              <'a>>) -> Self {Self::Fragment(value.into())}}
impl<'a> From<Option<FragmentQuery         <'a>>> for MaybeQueryLike<'a> {fn from(value: Option<FragmentQuery         <'a>>) -> Self {Self::Fragment(value.into())}}

impl<'a> From<MaybeQuery                   <'a>>  for MaybeQueryLike<'a> {fn from(value: MaybeQuery                   <'a> ) -> Self {Self::Query   (value       )}}
impl<'a> From<MaybeSpecialQuery            <'a>>  for MaybeQueryLike<'a> {fn from(value: MaybeSpecialQuery            <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<MaybeNonSpecialQuery         <'a>>  for MaybeQueryLike<'a> {fn from(value: MaybeNonSpecialQuery         <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<MaybeFragment                <'a>>  for MaybeQueryLike<'a> {fn from(value: MaybeFragment                <'a> ) -> Self {Self::Fragment(value.into())}}
impl<'a> From<MaybeFragmentQuery           <'a>>  for MaybeQueryLike<'a> {fn from(value: MaybeFragmentQuery           <'a> ) -> Self {Self::Fragment(value       )}}

impl<'a> From<QuerySegment                 <'a>>  for MaybeQueryLike<'a> {fn from(value: QuerySegment                 <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<SpecialQuerySegment          <'a>>  for MaybeQueryLike<'a> {fn from(value: SpecialQuerySegment          <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<NonSpecialQuerySegment       <'a>>  for MaybeQueryLike<'a> {fn from(value: NonSpecialQuerySegment       <'a> ) -> Self {Self::Query   (value.into())}}
impl<'a> From<FragmentQuerySegment         <'a>>  for MaybeQueryLike<'a> {fn from(value: FragmentQuerySegment         <'a> ) -> Self {Self::Fragment(value.into())}}

impl<'a> From<Option<SpecialQuerySegment   <'a>>> for MaybeQueryLike<'a> {fn from(value: Option<SpecialQuerySegment   <'a>>) -> Self {Self::Query   (value.into())}}
impl<'a> From<Option<NonSpecialQuerySegment<'a>>> for MaybeQueryLike<'a> {fn from(value: Option<NonSpecialQuerySegment<'a>>) -> Self {Self::Query   (value.into())}}
impl<'a> From<Option<FragmentQuerySegment  <'a>>> for MaybeQueryLike<'a> {fn from(value: Option<FragmentQuerySegment  <'a>>) -> Self {Self::Fragment(value.into())}}
