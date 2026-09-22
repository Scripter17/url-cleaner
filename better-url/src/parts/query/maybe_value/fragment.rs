//! [`MaybeFragmentQueryValue`].

use crate::prelude::*;

/// A [`FragmentQueryValue`] that might be [`None`].
#[derive(Debug, Clone)]
pub struct MaybeFragmentQueryValue<'a>(pub Option<FragmentQueryValue<'a>>);

impl<'a> MaybeFragmentQueryValue<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<T>) -> Self {
        Self(value.map(|x| unsafe {FragmentQueryValue::new_unchecked(x)}))
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(FragmentQueryValue::as_str)
    }

    /// [`FragmentQueryValue::decode`].
    pub fn decode(self) -> Option<Cow<'a, [u8]>> {
        Some(self.0?.decode())
    }

    /// [`FragmentQueryValue::try_decode`].
    /// # Errors
    /// If [`FragmentQueryValue::try_decode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fiiine.")]
    pub fn try_decode(self) -> Option<Result<Cow<'a, str>, (std::str::Utf8Error, Cow<'a, [u8]>)>> {
        Some(self.0?.try_decode())
    }

    /// [`FragmentQueryValue::lossy_decode`].
    pub fn lossy_decode(self) -> Option<Cow<'a, str>> {
        Some(self.0?.lossy_decode())
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybeFragmentQueryValue<'_> {
        MaybeFragmentQueryValue(self.0.as_ref().map(FragmentQueryValue::borrowed))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybeFragmentQueryValue<'static> {
        MaybeFragmentQueryValue(self.0.map(FragmentQueryValue::into_owned))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.0.map(FragmentQueryValue::into_inner)
    }
}



impl<'a> From<Option<Cow<'a, [u8]>>> for MaybeFragmentQueryValue<'a> {
    fn from(value: Option<Cow<'a, [u8]>>) -> Self {
        Self(value.map(Into::into))
    }
}



impl<'a> From<MaybeQueryLikeValue<'a>> for MaybeFragmentQueryValue<'a> {
    fn from(value: MaybeQueryLikeValue<'a>) -> Self {
        match value {
            MaybeQueryLikeValue::Query   (x) => x.into(),
            MaybeQueryLikeValue::Fragment(x) => x,
        }
    }
}

impl<'a> From<MaybeQueryValue<'a>> for MaybeFragmentQueryValue<'a> {
    fn from(value: MaybeQueryValue<'a>) -> Self {
        match value {
            MaybeQueryValue::Special   (x) => x.into(),
            MaybeQueryValue::NonSpecial(x) => x.into(),
        }
    }
}

impl<'a> From<MaybeSpecialQueryValue   <'a>> for MaybeFragmentQueryValue<'a> {fn from(value: MaybeSpecialQueryValue   <'a>) -> Self {value.0.into()}}
impl<'a> From<MaybeNonSpecialQueryValue<'a>> for MaybeFragmentQueryValue<'a> {fn from(value: MaybeNonSpecialQueryValue<'a>) -> Self {value.0.into()}}



impl<'a> From<Option<QueryLikeValue      <'a>>> for MaybeFragmentQueryValue<'a> {fn from(value: Option<QueryLikeValue      <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<FragmentQueryValue  <'a>>> for MaybeFragmentQueryValue<'a> {fn from(value: Option<FragmentQueryValue  <'a>>) -> Self {Self(value                )}}
impl<'a> From<Option<QueryValue          <'a>>> for MaybeFragmentQueryValue<'a> {fn from(value: Option<QueryValue          <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<SpecialQueryValue   <'a>>> for MaybeFragmentQueryValue<'a> {fn from(value: Option<SpecialQueryValue   <'a>>) -> Self {Self(value.map(Into::into))}}
impl<'a> From<Option<NonSpecialQueryValue<'a>>> for MaybeFragmentQueryValue<'a> {fn from(value: Option<NonSpecialQueryValue<'a>>) -> Self {Self(value.map(Into::into))}}
