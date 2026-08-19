//! [`MaybeNonSpecialQueryValue`].

use crate::prelude::*;

/// A [`NonSpecialQueryValue`] that might be [`None`].
#[derive(Debug, Clone)]
pub struct MaybeNonSpecialQueryValue<'a>(pub Option<NonSpecialQueryValue<'a>>);

impl<'a> MaybeNonSpecialQueryValue<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        Self(value.map(|x| unsafe {NonSpecialQueryValue::new_unchecked(x)}))
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(NonSpecialQueryValue::as_str)
    }

    /// [`NonSpecialQueryValue::decode`].
    pub fn decode(self) -> Option<Cow<'a, [u8]>> {
        Some(self.0?.decode())
    }

    /// [`NonSpecialQueryValue::try_decode`].
    /// # Errors
    /// If [`NonSpecialQueryValue::try_decode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fiiine.")]
    pub fn try_decode(self) -> Option<Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)>> {
        Some(self.0?.try_decode())
    }

    /// [`NonSpecialQueryValue::lossy_decode`].
    pub fn lossy_decode(self) -> Option<Cow<'a, str>> {
        Some(self.0?.lossy_decode())
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeNonSpecialQueryValue<'_> {
        MaybeNonSpecialQueryValue(self.0.as_ref().map(NonSpecialQueryValue::borrowed))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeNonSpecialQueryValue<'static> {
        MaybeNonSpecialQueryValue(self.0.map(NonSpecialQueryValue::into_owned))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(NonSpecialQueryValue::into_inner)
    }
}



impl<'a> From<Option<Cow<'a, str>>> for MaybeNonSpecialQueryValue<'a> {
    fn from(value: Option<Cow<'a, str>>) -> Self {
        Self(value.map(Into::into))
    }
}



impl<'a> From<MaybeQueryLikeValue<'a>> for MaybeNonSpecialQueryValue<'a> {
    fn from(value: MaybeQueryLikeValue<'a>) -> Self {
        match value {
            MaybeQueryLikeValue::Query   (x) => x.into(),
            MaybeQueryLikeValue::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeQueryValue<'a>> for MaybeNonSpecialQueryValue<'a> {
    fn from(value: MaybeQueryValue<'a>) -> Self {
        match value {
            MaybeQueryValue::Special   (x) => x.into(),
            MaybeQueryValue::NonSpecial(x) => x,
        }
    }
}

impl<'a> From<MaybeFragmentQueryValue<'a>> for MaybeNonSpecialQueryValue<'a> {fn from(value: MaybeFragmentQueryValue<'a>) -> Self {value.0.into()}}
impl<'a> From<MaybeSpecialQueryValue <'a>> for MaybeNonSpecialQueryValue<'a> {fn from(value: MaybeSpecialQueryValue <'a>) -> Self {value.0.into()}}



impl<'a> From<Option<QueryLikeValue      <'a>>> for MaybeNonSpecialQueryValue<'a> {fn from(value: Option<QueryLikeValue      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQueryValue  <'a>>> for MaybeNonSpecialQueryValue<'a> {fn from(value: Option<FragmentQueryValue  <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<QueryValue          <'a>>> for MaybeNonSpecialQueryValue<'a> {fn from(value: Option<QueryValue          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQueryValue   <'a>>> for MaybeNonSpecialQueryValue<'a> {fn from(value: Option<SpecialQueryValue   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQueryValue<'a>>> for MaybeNonSpecialQueryValue<'a> {fn from(value: Option<NonSpecialQueryValue<'a>>) -> Self {Self(value                )}}
