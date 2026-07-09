//! [`MaybeQueryLike`].

use crate::prelude::*;

mod get;
mod set;
mod filter;

/// The "mode" of a [`QueryLike`]/[`MaybeQueryLike`].
#[derive(Debug, Clone, Copy)]
pub(crate) enum QueryLikeMode {
    /// [`Query::Special`]/[`MaybeQuery::Special`],
    Special,
    /// [`Query::NonSpecial`]/[`MaybeQuery::NonSpecial`],
    NonSpecial,
    /// [`QueryLike::Fragment`]/[`MaybeQueryLike::Fragment`],
    Fragment,
}

/// Either [`MaybeQuery`] or [`MaybeFragmentQuery`].
#[derive(Debug, Clone)]
pub enum MaybeQueryLike<'a> {
    /// [`MaybeQuery`].
    Query(MaybeQuery<'a>),
    /// [`MaybeFragmentQuery`].
    Fragment(MaybeFragmentQuery<'a>),
}

impl<'a> MaybeQueryLike<'a> {
    /// Make a new [`Self`].
    pub fn new<T: Into<Self>>(value: T) -> Self {
        value.into()
    }

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
