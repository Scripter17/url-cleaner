//! [`Fragment`].

use crate::prelude::*;

/// A fragment string.
#[derive(Debug, Clone)]
pub struct Fragment<'a>(pub(crate) Cow<'a, str>);

impl<'a> Fragment<'a> {
    /// Make a new [`Self`] without checking for validity.
    pub(crate) fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Turn into a [`FragmentQuery`].
    pub fn query(self) -> FragmentQuery<'a> {
        self.into()
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Fragment<'static> {
        Fragment(self.0.into_owned().into())
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Fragment<'_> {
        Fragment(Cow::Borrowed(&self.0))
    }
}

impl<'a> From<Cow<'a, str>> for Fragment<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(encode_fragment(value).1)
    }
}

impl<'a> From<Query<'a>> for Fragment<'a> {
    fn from(value: Query<'a>) -> Self {
        match value {
            Query::Special   (x) => x.into(),
            Query::NonSpecial(x) => x.into(),
            Query::Fragment  (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuery   <'a>> for Fragment<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(special_query_to_fragment    (value.into_inner()).1)}}
impl<'a> From<NonSpecialQuery<'a>> for Fragment<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(non_special_query_to_fragment(value.into_inner()).1)}}
impl<'a> From<FragmentQuery  <'a>> for Fragment<'a> {fn from(value: FragmentQuery  <'a>) -> Self {Self(                              value.into_inner()   )}}

impl<'a> From<QuerySegment<'a>> for Fragment<'a> {
    fn from(value: QuerySegment<'a>) -> Self {
        match value {
            QuerySegment::Special   (x) => x.into(),
            QuerySegment::NonSpecial(x) => x.into(),
            QuerySegment::Fragment  (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialQuerySegment   <'a>> for Fragment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(special_query_to_fragment    (value.into_inner()).1)}}
impl<'a> From<NonSpecialQuerySegment<'a>> for Fragment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(non_special_query_to_fragment(value.into_inner()).1)}}
impl<'a> From<FragmentQuerySegment  <'a>> for Fragment<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self(                              value.into_inner()   )}}
