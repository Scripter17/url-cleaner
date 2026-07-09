//! [`FragmentQuery`].

use crate::prelude::*;

mod get;
mod set;

/// A special query.
#[derive(Debug, Clone)]
pub struct FragmentQuery<'a>(pub(crate) Cow<'a, str>);

impl<'a> FragmentQuery<'a> {
    /// Make a new [`Self`] without checking for validity.
    pub(crate) fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into a [`Fragment`].
    pub fn string(self) -> Fragment<'a> {
        self.into()
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> FragmentQuery<'static> {
        FragmentQuery(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> FragmentQuery<'_> {
        FragmentQuery(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for FragmentQuery<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_fragment(value).1)
    }
}



impl<'a> From<QueryLike<'a>> for FragmentQuery<'a> {
    fn from(value: QueryLike<'a>) -> Self {
        match value {
            QueryLike::Query   (x) => x.into(),
            QueryLike::Fragment(x) => x,
        }
    }
}

impl<'a> From<Query<'a>> for FragmentQuery<'a> {
    fn from(value: Query<'a>) -> Self {
        match value {
            Query::Special   (x) => x.into(),
            Query::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuery   <'a>> for FragmentQuery<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(special_query_to_fragment    (value.into_inner()).1)}}
impl<'a> From<NonSpecialQuery<'a>> for FragmentQuery<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(non_special_query_to_fragment(value.into_inner()).1)}}
impl<'a> From<Fragment       <'a>> for FragmentQuery<'a> {fn from(value: Fragment       <'a>) -> Self {Self(                              value.into_inner()   )}}



impl<'a> From<QueryLikeSegment<'a>> for FragmentQuery<'a> {
    fn from(value: QueryLikeSegment<'a>) -> Self {
        match value {
            QueryLikeSegment::Query   (x) => x.into(),
            QueryLikeSegment::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<QuerySegment<'a>> for FragmentQuery<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuerySegment   <'a>> for FragmentQuery<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(special_query_to_fragment    (value.into_inner()).1)}}
impl<'a> From<NonSpecialQuerySegment<'a>> for FragmentQuery<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(non_special_query_to_fragment(value.into_inner()).1)}}
impl<'a> From<FragmentQuerySegment  <'a>> for FragmentQuery<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self(                              value.into_inner()   )}}
