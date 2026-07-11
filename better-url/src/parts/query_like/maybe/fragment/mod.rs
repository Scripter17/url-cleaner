//! [`MaybeFragmentQuery`].

use crate::prelude::*;

mod get;
mod set;
mod filter;

/// A [`FragmentQuery`] that might be [`None`].
#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct MaybeFragmentQuery<'a>(pub Option<FragmentQuery<'a>>);

impl<'a> MaybeFragmentQuery<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(FragmentQuery::as_str)
    }

    /// If [`Self::0`] is [`Some`].
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// If [`Self::0`] is [`None`].
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Turn into a [`Fragment`].
    pub fn string(self) -> MaybeFragment<'a> {
        self.into()
    }



    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        unsafe {
            Self(value.map(|x| FragmentQuery::new_unchecked(x)))
        }
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(FragmentQuery::into_inner)
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeFragmentQuery<'static> {
        MaybeFragmentQuery(self.0.map(FragmentQuery::into_owned))
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeFragmentQuery<'_> {
        MaybeFragmentQuery(self.0.as_ref().map(FragmentQuery::borrowed))
    }
}

impl<'a> From<Option<Cow<'a, str>>> for MaybeFragmentQuery<'a> {fn from(value: Option<Cow<'a, str>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<MaybeQueryLike<'a>> for MaybeFragmentQuery<'a> {
    fn from(value: MaybeQueryLike<'a>) -> Self {
        match value {
            MaybeQueryLike::Query   (x) => x.into(),
            MaybeQueryLike::Fragment(x) => x,
        }
    }
}

impl<'a> From<MaybeQuery<'a>> for MaybeFragmentQuery<'a> {
    fn from(value: MaybeQuery<'a>) -> Self {
        match value {
            MaybeQuery::Special   (x) => x.into(),
            MaybeQuery::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<Query          <'a>> for MaybeFragmentQuery<'a> {fn from(value: Query          <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<QueryLike      <'a>> for MaybeFragmentQuery<'a> {fn from(value: QueryLike      <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<SpecialQuery   <'a>> for MaybeFragmentQuery<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuery<'a>> for MaybeFragmentQuery<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<FragmentQuery  <'a>> for MaybeFragmentQuery<'a> {fn from(value: FragmentQuery  <'a>) -> Self {Self(Some(value))}}

impl<'a> From<MaybeSpecialQuery   <'a>> for MaybeFragmentQuery<'a> {fn from(value: MaybeSpecialQuery   <'a>) -> Self {Self(value.0.map(Into::into))}}
impl<'a> From<MaybeNonSpecialQuery<'a>> for MaybeFragmentQuery<'a> {fn from(value: MaybeNonSpecialQuery<'a>) -> Self {Self(value.0.map(Into::into))}}

impl<'a> From<QuerySegment          <'a>> for MaybeFragmentQuery<'a> {fn from(value: QuerySegment          <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<QueryLikeSegment      <'a>> for MaybeFragmentQuery<'a> {fn from(value: QueryLikeSegment      <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuerySegment<'a>> for MaybeFragmentQuery<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<SpecialQuerySegment   <'a>> for MaybeFragmentQuery<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<FragmentQuerySegment  <'a>> for MaybeFragmentQuery<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<Query          <'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<Query          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<QueryLike      <'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<QueryLike      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuery   <'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<SpecialQuery   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuery<'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<NonSpecialQuery<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuery  <'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<FragmentQuery  <'a>>) -> Self {Self(value)}}

impl<'a> From<Option<QuerySegment          <'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<QuerySegment          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuerySegment<'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<NonSpecialQuerySegment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuerySegment   <'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<SpecialQuerySegment   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuerySegment  <'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<FragmentQuerySegment  <'a>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<Fragment       <'a> > for MaybeFragmentQuery<'a> {fn from(value: Fragment       <'a> ) -> Self {Self(Some(value.into()))}}
impl<'a> From<Option<Fragment<'a>>> for MaybeFragmentQuery<'a> {fn from(value: Option<Fragment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<MaybeFragment  <'a> > for MaybeFragmentQuery<'a> {fn from(value: MaybeFragment  <'a> ) -> Self {Self(value.0.map(Into::into))}}
