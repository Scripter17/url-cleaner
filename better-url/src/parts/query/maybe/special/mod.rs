//! [`MaybeSpecialQuery`].

use crate::prelude::*;

mod get;
mod set;
mod filter;

/// A [`SpecialQuery`] that might be [`None`].
#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct MaybeSpecialQuery<'a>(pub Option<SpecialQuery<'a>>);

impl<'a> MaybeSpecialQuery<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(SpecialQuery::as_str)
    }

    /// If [`Self::0`] is [`Some`].
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// If [`Self::0`] is [`None`].
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// The length of the [`BetterUrl::canon_get_search`] for this value.
    pub fn search_len(&self) -> usize {
        self.len().map_or(0, |x| x + 1)
    }



    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        unsafe {
            Self(value.map(|x| SpecialQuery::new_unchecked(x)))
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeSpecialQuery<'_> {
        MaybeSpecialQuery(self.0.as_ref().map(SpecialQuery::borrowed))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeSpecialQuery<'static> {
        MaybeSpecialQuery(self.0.map(SpecialQuery::into_owned))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(SpecialQuery::into_inner)
    }
}

impl<'a> From<Option<Cow<'a, str>>> for MaybeSpecialQuery<'a> {fn from(value: Option<Cow<'a, str>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<MaybeQueryLike<'a>> for MaybeSpecialQuery<'a> {
    fn from(value: MaybeQueryLike<'a>) -> Self {
        match value {
            MaybeQueryLike::Query   (x) => x.into(),
            MaybeQueryLike::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeQuery<'a>> for MaybeSpecialQuery<'a> {
    fn from(value: MaybeQuery<'a>) -> Self {
        match value {
            MaybeQuery::Special   (x) => x,
            MaybeQuery::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<Option<QueryLike             <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<QueryLike             <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuery         <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<FragmentQuery         <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<Fragment              <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<Fragment              <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<Query                 <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<Query                 <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuery          <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<SpecialQuery          <'a>>) -> Self {Self(value                )}}
impl<'a> From<Option<NonSpecialQuery       <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<NonSpecialQuery       <'a>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<MaybeFragmentQuery           <'a> > for MaybeSpecialQuery<'a> {fn from(value: MaybeFragmentQuery           <'a> ) -> Self {value.0.into()}}
impl<'a> From<MaybeFragment                <'a> > for MaybeSpecialQuery<'a> {fn from(value: MaybeFragment                <'a> ) -> Self {value.0.into()}}
impl<'a> From<MaybeNonSpecialQuery         <'a> > for MaybeSpecialQuery<'a> {fn from(value: MaybeNonSpecialQuery         <'a> ) -> Self {value.0.into()}}

impl<'a> From<Option<QueryLikeSegment      <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<QueryLikeSegment      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuerySegment  <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<FragmentQuerySegment  <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<QuerySegment          <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<QuerySegment          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuerySegment<'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<NonSpecialQuerySegment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuerySegment   <'a>>> for MaybeSpecialQuery<'a> {fn from(value: Option<SpecialQuerySegment   <'a>>) -> Self {Self(value.map(Into::into))}}
