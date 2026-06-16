//! [`MaybeQuery`].

use crate::prelude::*;

mod get;
mod set;
mod filter;

/// A [`Query`] that might be [`None`].
#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct MaybeQuery<'a>(pub Option<Query<'a>>);

impl<'a> MaybeQuery<'a> {
    /// [`Query::Special`].
    pub fn new_special<T: Into<MaybeSpecialQuery<'a>>>(query: T) -> Self {
        query.into().into()
    }

    /// [`Query::NonSpecial`].
    pub fn new_non_special<T: Into<MaybeNonSpecialQuery<'a>>>(query: T) -> Self {
        query.into().into()
    }

    /// [`Query::Fragment`].
    pub fn new_fragment<T: Into<MaybeFragmentQuery<'a>>>(query: T) -> Self {
        query.into().into()
    }



    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(Query::as_str)
    }

    /// If [`Self::0`] is [`Some`].
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// If [`Self::0`] is [`None`].
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(Query::into_inner)
    }

    /// Turn into an owned [`MaybeQuery`].
    pub fn into_owned(self) -> MaybeQuery<'static> {
        MaybeQuery(self.0.map(Query::into_owned))
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeQuery<'_> {
        MaybeQuery(self.0.as_ref().map(Query::borrowed))
    }
}

impl<'a> From<Query          <'a>> for MaybeQuery<'a> {fn from(value: Query          <'a>) -> Self {Self(Some(value))}}
impl<'a> From<SpecialQuery   <'a>> for MaybeQuery<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuery<'a>> for MaybeQuery<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<FragmentQuery  <'a>> for MaybeQuery<'a> {fn from(value: FragmentQuery  <'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<Query          <'a>>> for MaybeQuery<'a> {fn from(value: Option<Query          <'a>>) -> Self {Self(value)}}
impl<'a> From<Option<SpecialQuery   <'a>>> for MaybeQuery<'a> {fn from(value: Option<SpecialQuery   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuery<'a>>> for MaybeQuery<'a> {fn from(value: Option<NonSpecialQuery<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuery  <'a>>> for MaybeQuery<'a> {fn from(value: Option<FragmentQuery  <'a>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<MaybeSpecialQuery   <'a>> for MaybeQuery<'a> {fn from(value: MaybeSpecialQuery   <'a>) -> Self {Self(value.0.map(Into::into))}}
impl<'a> From<MaybeNonSpecialQuery<'a>> for MaybeQuery<'a> {fn from(value: MaybeNonSpecialQuery<'a>) -> Self {Self(value.0.map(Into::into))}}
impl<'a> From<MaybeFragmentQuery  <'a>> for MaybeQuery<'a> {fn from(value: MaybeFragmentQuery  <'a>) -> Self {Self(value.0.map(Into::into))}}

impl<'a> From<QuerySegment          <'a>> for MaybeQuery<'a> {fn from(value: QuerySegment          <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<SpecialQuerySegment   <'a>> for MaybeQuery<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuerySegment<'a>> for MaybeQuery<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<FragmentQuerySegment  <'a>> for MaybeQuery<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<QuerySegment          <'a>>> for MaybeQuery<'a> {fn from(value: Option<QuerySegment          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuerySegment   <'a>>> for MaybeQuery<'a> {fn from(value: Option<SpecialQuerySegment   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuerySegment<'a>>> for MaybeQuery<'a> {fn from(value: Option<NonSpecialQuerySegment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuerySegment  <'a>>> for MaybeQuery<'a> {fn from(value: Option<FragmentQuerySegment  <'a>>) -> Self {Self(value.map(Into::into))}}


impl<'a> From<Fragment       <'a> > for MaybeQuery<'a> {fn from(value: Fragment       <'a> ) -> Self {Self(Some(value.into()))}}
impl<'a> From<Option<Fragment<'a>>> for MaybeQuery<'a> {fn from(value: Option<Fragment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<MaybeFragment  <'a> > for MaybeQuery<'a> {fn from(value: MaybeFragment  <'a> ) -> Self {Self(value.0.map(Into::into))}}
