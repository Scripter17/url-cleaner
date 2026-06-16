//! [`SpecialQuery`].

use crate::prelude::*;

mod get;
mod set;

/// A special query.
#[derive(Debug, Clone)]
pub struct SpecialQuery<'a>(pub(crate) Cow<'a, str>);

impl<'a> SpecialQuery<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> SpecialQuery<'static> {
        SpecialQuery(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> SpecialQuery<'_> {
        SpecialQuery(Cow::Borrowed(&self.0))
    }
}



impl<'a> From<Cow<'a, str>> for SpecialQuery<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_special_query(value).1)
    }
}



impl<'a> From<Query<'a>> for SpecialQuery<'a> {
    fn from(value: Query<'a>) -> Self {
        match value {
            Query::Special   (x) => x,
            Query::NonSpecial(x) => x.into(),
            Query::Fragment  (x) => x.into(),
        }
    }
}

impl<'a> From<NonSpecialQuery<'a>> for SpecialQuery<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(non_special_query_to_special_query(value.0).1)}}
impl<'a> From<FragmentQuery  <'a>> for SpecialQuery<'a> {fn from(value: FragmentQuery  <'a>) -> Self {Self(fragment_to_special_query         (value.0).1)}}



impl<'a> From<QuerySegment<'a>> for SpecialQuery<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x.into(),
            QuerySegment::Fragment  (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuerySegment   <'a>> for SpecialQuery<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(                                   value.into_inner()   )}}
impl<'a> From<NonSpecialQuerySegment<'a>> for SpecialQuery<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(non_special_query_to_special_query(value.into_inner()).1)}}
impl<'a> From<FragmentQuerySegment  <'a>> for SpecialQuery<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self(fragment_to_special_query         (value.into_inner()).1)}}


impl<'a> From<Fragment<'a>> for SpecialQuery<'a> {fn from(value: Fragment<'a>) -> Self {value.query().into()}}
