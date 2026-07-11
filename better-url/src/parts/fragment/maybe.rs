//! [`MaybeFragment`].

use crate::prelude::*;

/// A [`Fragment`] that might be [`None`].
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MaybeFragment<'a>(pub Option<Fragment<'a>>);

impl<'a> MaybeFragment<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        unsafe {
            Self(value.map(|x| Fragment::new_unchecked(x)))
        }
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(Fragment::as_str)
    }

    /// Turn into a [`MaybeQuery`].
    pub fn query(self) -> MaybeFragmentQuery<'a> {
        self.into()
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

impl<'a> From<Fragment<'a>> for MaybeFragment<'a> {
    fn from(value: Fragment<'a>) -> Self {
        Self(Some(value))
    }
}

impl<'a> From<Query          <'a>> for MaybeFragment<'a> {fn from(value: Query          <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<QueryLike      <'a>> for MaybeFragment<'a> {fn from(value: QueryLike      <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<SpecialQuery   <'a>> for MaybeFragment<'a> {fn from(value: SpecialQuery   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuery<'a>> for MaybeFragment<'a> {fn from(value: NonSpecialQuery<'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<FragmentQuery  <'a>> for MaybeFragment<'a> {fn from(value: FragmentQuery  <'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<Query          <'a>>> for MaybeFragment<'a> {fn from(value: Option<Query          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<QueryLike      <'a>>> for MaybeFragment<'a> {fn from(value: Option<QueryLike      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuery   <'a>>> for MaybeFragment<'a> {fn from(value: Option<SpecialQuery   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuery<'a>>> for MaybeFragment<'a> {fn from(value: Option<NonSpecialQuery<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuery  <'a>>> for MaybeFragment<'a> {fn from(value: Option<FragmentQuery  <'a>>) -> Self {Self(value.map(Into::into))}}

impl<'a> From<MaybeQuery<'a>> for MaybeFragment<'a> {
    fn from(value: MaybeQuery<'a>) -> Self {
        match value {
            MaybeQuery::Special   (x) => x.into(),
            MaybeQuery::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeQueryLike<'a>> for MaybeFragment<'a> {
    fn from(value: MaybeQueryLike<'a>) -> Self {
        match value {
            MaybeQueryLike::Query   (x) => x.into(),
            MaybeQueryLike::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeSpecialQuery   <'a>> for MaybeFragment<'a> {fn from(value: MaybeSpecialQuery   <'a>) -> Self {value.0.into()}}
impl<'a> From<MaybeNonSpecialQuery<'a>> for MaybeFragment<'a> {fn from(value: MaybeNonSpecialQuery<'a>) -> Self {value.0.into()}}
impl<'a> From<MaybeFragmentQuery  <'a>> for MaybeFragment<'a> {fn from(value: MaybeFragmentQuery  <'a>) -> Self {value.0.into()}}

impl<'a> From<QuerySegment          <'a>> for MaybeFragment<'a> {fn from(value: QuerySegment          <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<QueryLikeSegment      <'a>> for MaybeFragment<'a> {fn from(value: QueryLikeSegment      <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<SpecialQuerySegment   <'a>> for MaybeFragment<'a> {fn from(value: SpecialQuerySegment   <'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<NonSpecialQuerySegment<'a>> for MaybeFragment<'a> {fn from(value: NonSpecialQuerySegment<'a>) -> Self {Self(Some(value.into()))}}
impl<'a> From<FragmentQuerySegment  <'a>> for MaybeFragment<'a> {fn from(value: FragmentQuerySegment  <'a>) -> Self {Self(Some(value.into()))}}

impl<'a> From<Option<QuerySegment          <'a>>> for MaybeFragment<'a> {fn from(value: Option<QuerySegment          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<QueryLikeSegment      <'a>>> for MaybeFragment<'a> {fn from(value: Option<QueryLikeSegment      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQuerySegment   <'a>>> for MaybeFragment<'a> {fn from(value: Option<SpecialQuerySegment   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQuerySegment<'a>>> for MaybeFragment<'a> {fn from(value: Option<NonSpecialQuerySegment<'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQuerySegment  <'a>>> for MaybeFragment<'a> {fn from(value: Option<FragmentQuerySegment  <'a>>) -> Self {Self(value.map(Into::into))}}
