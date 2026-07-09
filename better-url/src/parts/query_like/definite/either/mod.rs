//! [`QueryLike`].

use crate::prelude::*;

mod get;
mod set;

/// Either [`Query`] or [`FragmentQuery`].
#[derive(Debug, Clone)]
pub enum QueryLike<'a> {
    /// [`Query`].
    Query(Query<'a>),
    /// [`FragmentQuery`].
    Fragment(FragmentQuery<'a>),
}

impl<'a> QueryLike<'a> {
    /// Either [`Query::as_str`] or [`FragmentQuery::as_str`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Query   (x) => x.as_str(),
            Self::Fragment(x) => x.as_str(),
        }
    }



    /// Either [`Query::into_inner`] or [`FragmentQuery::into_inner`].
    pub fn into_inner(self) -> Cow<'a, str> {
        match self {
            Self::Query   (x) => x.into_inner(),
            Self::Fragment(x) => x.into_inner(),
        }
    }

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
