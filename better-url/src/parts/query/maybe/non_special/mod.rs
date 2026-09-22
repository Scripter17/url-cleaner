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
            Self(value.map(|x| NonSpecialQuery::new_unchecked(x)))
        }
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeNonSpecialQuery<'_> {
        MaybeNonSpecialQuery(self.0.as_ref().map(NonSpecialQuery::borrowed))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeNonSpecialQuery<'static> {
        MaybeNonSpecialQuery(self.0.map(NonSpecialQuery::into_owned))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(NonSpecialQuery::into_inner)
    }
}



impl<'a> From<Option<Cow<'a, [u8]>>> for MaybeNonSpecialQuery<'a> {
    fn from(value: Option<Cow<'a, [u8]>>) -> Self {
        Self(value.map(Into::into))
    }
}

impl<'a> From<MaybeQueryLike<'a>> for MaybeNonSpecialQuery<'a> {
    fn from(value: MaybeQueryLike<'a>) -> Self {
        match value {
            MaybeQueryLike::Query   (x) => x.into(),
            MaybeQueryLike::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeQuery<'a>> for MaybeNonSpecialQuery<'a> {
    fn from(value: MaybeQuery<'a>) -> Self {
        match value {
            MaybeQuery::Special   (x) => x.into(),
            MaybeQuery::NonSpecial(x) => x,
        }
    }
}

impl<'a> From<Option<QueryLike             <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<QueryLike             <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuery         <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<FragmentQuery         <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<Fragment              <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<Fragment              <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<Query                 <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<Query                 <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuery          <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<SpecialQuery          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuery       <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<NonSpecialQuery       <'a>>) -> Self {Self(value                )}}

impl<'a> From<MaybeFragmentQuery           <'a> > for MaybeNonSpecialQuery<'a> {fn from(value: MaybeFragmentQuery           <'a> ) -> Self {value.0.into()}}
impl<'a> From<MaybeFragment                <'a> > for MaybeNonSpecialQuery<'a> {fn from(value: MaybeFragment                <'a> ) -> Self {value.0.into()}}
impl<'a> From<MaybeSpecialQuery            <'a> > for MaybeNonSpecialQuery<'a> {fn from(value: MaybeSpecialQuery            <'a> ) -> Self {value.0.into()}}

impl<'a> From<Option<QueryLikeSegment      <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<QueryLikeSegment      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuerySegment  <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<FragmentQuerySegment  <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<QuerySegment          <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<QuerySegment          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuerySegment<'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<NonSpecialQuerySegment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuerySegment   <'a>>> for MaybeNonSpecialQuery<'a> {fn from(value: Option<SpecialQuerySegment   <'a>>) -> Self {Self(value.map(Into::into))}}
