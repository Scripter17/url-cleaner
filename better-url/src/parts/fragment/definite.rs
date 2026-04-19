//! [`Fragment`].

use crate::prelude::*;

/// A fragment string.
#[derive(Debug, Clone)]
pub struct Fragment<'a>(pub(crate) Cow<'a, str>);

impl<'a> Fragment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
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
        Self(PartTranscoder::Fragment.encode(value))
    }
}

impl<'a> From<Query          <'a>> for Fragment<'a> {fn from(value: Query          <'a>) -> Self {Self(query_to_fragment(value.into_inner()))}}
impl<'a> From<SpecialQuery   <'a>> for Fragment<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(query_to_fragment(value.into_inner()))}}
impl<'a> From<NonSpecialQuery<'a>> for Fragment<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(query_to_fragment(value.into_inner()))}}

impl<'a> From<QuerySegment          <'a>> for Fragment<'a> {fn from(value: QuerySegment          <'a>) -> Self {Self(query_to_fragment(value.into_inner()))}}
impl<'a> From<SpecialQuerySegment   <'a>> for Fragment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(query_to_fragment(value.into_inner()))}}
impl<'a> From<NonSpecialQuerySegment<'a>> for Fragment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(query_to_fragment(value.into_inner()))}}
