//! [`MaybeSpecialQueryValue`].

use crate::prelude::*;

/// A [`SpecialQueryValue`] that might be [`None`].
#[derive(Debug, Clone)]
pub struct MaybeSpecialQueryValue<'a>(pub Option<SpecialQueryValue<'a>>);

impl<'a> MaybeSpecialQueryValue<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        Self(value.map(|x| unsafe {SpecialQueryValue::new_unchecked(x)}))
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(SpecialQueryValue::as_str)
    }

    /// [`SpecialQueryValue::decode`].
    pub fn decode(self) -> Option<Cow<'a, [u8]>> {
        Some(self.0?.decode())
    }

    /// [`SpecialQueryValue::try_decode`].
    /// # Errors
    /// If [`SpecialQueryValue::try_decode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fiiine.")]
    pub fn try_decode(self) -> Option<Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)>> {
        Some(self.0?.try_decode())
    }

    /// [`SpecialQueryValue::lossy_decode`].
    pub fn lossy_decode(self) -> Option<Cow<'a, str>> {
        Some(self.0?.lossy_decode())
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeSpecialQueryValue<'_> {
        MaybeSpecialQueryValue(self.0.as_ref().map(SpecialQueryValue::borrowed))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeSpecialQueryValue<'static> {
        MaybeSpecialQueryValue(self.0.map(SpecialQueryValue::into_owned))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(SpecialQueryValue::into_inner)
    }
}



impl<'a> From<Option<Cow<'a, str>>> for MaybeSpecialQueryValue<'a> {
    fn from(value: Option<Cow<'a, str>>) -> Self {
        Self(value.map(Into::into))
    }
}



impl<'a> From<MaybeQueryLikeValue<'a>> for MaybeSpecialQueryValue<'a> {
    fn from(value: MaybeQueryLikeValue<'a>) -> Self {
        match value {
            MaybeQueryLikeValue::Query   (x) => x.into(),
            MaybeQueryLikeValue::Fragment(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeQueryValue<'a>> for MaybeSpecialQueryValue<'a> {
    fn from(value: MaybeQueryValue<'a>) -> Self {
        match value {
            MaybeQueryValue::Special   (x) => x,
            MaybeQueryValue::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeFragmentQueryValue  <'a>> for MaybeSpecialQueryValue<'a> {fn from(value: MaybeFragmentQueryValue  <'a>) -> Self {value.0.into()}}
impl<'a> From<MaybeNonSpecialQueryValue<'a>> for MaybeSpecialQueryValue<'a> {fn from(value: MaybeNonSpecialQueryValue<'a>) -> Self {value.0.into()}}



impl<'a> From<Option<QueryLikeValue      <'a>>> for MaybeSpecialQueryValue<'a> {fn from(value: Option<QueryLikeValue      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQueryValue  <'a>>> for MaybeSpecialQueryValue<'a> {fn from(value: Option<FragmentQueryValue  <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<QueryValue          <'a>>> for MaybeSpecialQueryValue<'a> {fn from(value: Option<QueryValue          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQueryValue   <'a>>> for MaybeSpecialQueryValue<'a> {fn from(value: Option<SpecialQueryValue   <'a>>) -> Self {Self(value                )}}
impl<'a> From<Option<NonSpecialQueryValue<'a>>> for MaybeSpecialQueryValue<'a> {fn from(value: Option<NonSpecialQueryValue<'a>>) -> Self {Self(value.map(Into::into))}}
