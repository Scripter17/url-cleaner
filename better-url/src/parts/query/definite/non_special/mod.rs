//! [`NonSpecialQuery`].

use crate::prelude::*;

mod get;
mod set;

/// A non-special query.
#[derive(Debug, Clone)]
pub struct NonSpecialQuery<'a>(pub(crate) Cow<'a, str>);

impl<'a> NonSpecialQuery<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> NonSpecialQuery<'_> {
        NonSpecialQuery(Cow::Borrowed(&self.0))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> NonSpecialQuery<'static> {
        NonSpecialQuery(self.0.into_owned().into())
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}



impl<'a> From<Cow<'a, str>> for NonSpecialQuery<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_non_special_query(value).1)
    }
}



impl<'a> From<QueryLike<'a>> for NonSpecialQuery<'a> {
    fn from(value: QueryLike<'a>) -> Self {
        match value {
            QueryLike::Query   (x) => x.into(),
            QueryLike::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<Query<'a>> for NonSpecialQuery<'a> {
    fn from(value: Query<'a>) -> Self {
        match value {
            Query::Special   (x) => x.into(),
            Query::NonSpecial(x) => x,
        }
    }
}

impl<'a> From<QueryLikeSegment<'a>> for NonSpecialQuery<'a> {
    fn from(value: QueryLikeSegment<'a>) -> Self {
        match value {
            QueryLikeSegment::Query   (x) => x.into(),
            QueryLikeSegment::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<QuerySegment<'a>> for NonSpecialQuery<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuery <'a>> for NonSpecialQuery<'a> {fn from(value: SpecialQuery <'a>) -> Self {Self(                              value.into_inner()   )}}
impl<'a> From<Fragment     <'a>> for NonSpecialQuery<'a> {fn from(value: Fragment     <'a>) -> Self {Self(fragment_to_non_special_query(value.into_inner()).1)}}
impl<'a> From<FragmentQuery<'a>> for NonSpecialQuery<'a> {fn from(value: FragmentQuery<'a>) -> Self {Self(fragment_to_non_special_query(value.into_inner()).1)}}

impl<'a> From<SpecialQuerySegment   <'a>> for NonSpecialQuery<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(                              value.into_inner()   )}}
impl<'a> From<NonSpecialQuerySegment<'a>> for NonSpecialQuery<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(                              value.into_inner()   )}}
impl<'a> From<FragmentQuerySegment  <'a>> for NonSpecialQuery<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self(fragment_to_non_special_query(value.into_inner()).1)}}
