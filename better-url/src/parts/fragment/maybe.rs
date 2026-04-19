//! [`MaybeFragment`].

use crate::prelude::*;

/// A [`Fragment`] that might be [`None`].
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MaybeFragment<'a>(pub Option<Fragment<'a>>);

impl<'a> MaybeFragment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(Fragment::as_str)
    }

    /// Turn into a [`MaybeQuery`].
    pub fn into_query(self) -> MaybeQuery<'a> {
        self.into()
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(Fragment::into_inner)
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeFragment<'static> {
        MaybeFragment(self.0.map(Fragment::into_owned))
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeFragment<'_> {
        MaybeFragment(self.0.as_ref().map(Fragment::borrowed))
    }
}

impl<'a> From<Option<Cow<'a, str>>> for MaybeFragment<'a> {
    fn from(value: Option<Cow<'a, str>>) -> Self {
        Self(value.map(Into::into))
    }
}

impl<'a> From<QuerySegment   <'a>> for MaybeFragment<'a> {fn from(value: QuerySegment   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<Query          <'a>> for MaybeFragment<'a> {fn from(value: Query          <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<SpecialQuery   <'a>> for MaybeFragment<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuery<'a>> for MaybeFragment<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<QuerySegment   <'a>>> for MaybeFragment<'a> {fn from(value: Option<QuerySegment   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<Query          <'a>>> for MaybeFragment<'a> {fn from(value: Option<Query          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuery   <'a>>> for MaybeFragment<'a> {fn from(value: Option<SpecialQuery   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuery<'a>>> for MaybeFragment<'a> {fn from(value: Option<NonSpecialQuery<'a>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<MaybeQuery          <'a>> for MaybeFragment<'a> {fn from(value: MaybeQuery          <'a>) -> Self {value.0.into()}}
impl<'a> From<MaybeSpecialQuery   <'a>> for MaybeFragment<'a> {fn from(value: MaybeSpecialQuery   <'a>) -> Self {value.0.into()}}
impl<'a> From<MaybeNonSpecialQuery<'a>> for MaybeFragment<'a> {fn from(value: MaybeNonSpecialQuery<'a>) -> Self {value.0.into()}}
