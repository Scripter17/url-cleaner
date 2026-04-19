//! [`MaybeNonSpecialQuery`].

use crate::prelude::*;

mod get;
mod set;
mod filter;

/// A [`NonSpecialQuery`] that might be [`None`].
#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct MaybeNonSpecialQuery<'a>(pub Option<NonSpecialQuery<'a>>);

impl<'a> MaybeNonSpecialQuery<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(NonSpecialQuery::as_str)
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(NonSpecialQuery::into_inner)
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeNonSpecialQuery<'static> {
        MaybeNonSpecialQuery(self.0.map(NonSpecialQuery::into_owned))
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeNonSpecialQuery<'_> {
        MaybeNonSpecialQuery(self.0.as_ref().map(NonSpecialQuery::borrowed))
    }
}

impl<'a> From<Option<Cow<'a, str>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<Cow<'a, str>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<NonSpecialQuerySegment   <'a>> for MaybeNonSpecialQuery<'a> {fn from(value: NonSpecialQuerySegment   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<Query          <'a>> for MaybeNonSpecialQuery<'a> {fn from(value: Query          <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<SpecialQuery   <'a>> for MaybeNonSpecialQuery<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuery<'a>> for MaybeNonSpecialQuery<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(Some(value))}}

impl<'a> From<Option<NonSpecialQuerySegment   <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<NonSpecialQuerySegment   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<Query          <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<Query          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuery   <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<SpecialQuery   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuery<'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<NonSpecialQuery<'a>>) -> Self {Self(value)}}

impl<'a> From<MaybeQuery       <'a>> for MaybeNonSpecialQuery<'a> {fn from(value: MaybeQuery       <'a>) -> Self {Self(value.0.map(Into::into))}}
impl<'a> From<MaybeSpecialQuery<'a>> for MaybeNonSpecialQuery<'a> {fn from(value: MaybeSpecialQuery<'a>) -> Self {Self(value.0.map(Into::into))}}



impl<'a> From<Fragment       <'a> > for MaybeNonSpecialQuery<'a> {fn from(value: Fragment       <'a> ) -> Self {Self(Some(value.into()))}}
impl<'a> From<Option<Fragment<'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<Fragment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<MaybeFragment  <'a> > for MaybeNonSpecialQuery<'a> {fn from(value: MaybeFragment  <'a> ) -> Self {Self(value.0.map(Into::into))}}
