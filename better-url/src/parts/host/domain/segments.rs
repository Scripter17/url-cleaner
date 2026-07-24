//! [`DomainSegments`].

use crate::prelude::*;

/// A sequence of domain segments.
#[derive(Debug, Clone)]
pub struct DomainSegments<'a>(pub(crate) Cow<'a, str>);

impl<'a> DomainSegments<'a> {
    /// Make a new [`Self`] with zero validity checks.
    /// # Safety
    /// `value` must be a valid domain segments literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// [`unchecked_decode_domain_segments`].
    pub fn decode(self) -> Cow<'a, str> {
        let (_, value) = unchecked_decode_domain_segments(self.0);

        value
    }



    /// A [`DomainSegmentsIter`].
    pub fn iter(&self) -> DomainSegmentsIter<'_> {
        self.into_iter()
    }

    /// If it [`ends_in_a_number`].
    pub fn ends_in_a_number(&self) -> bool {
        ends_in_a_number(self.as_str())
    }

    /// [`last_is_empty`].
    pub fn last_is_empty(&self) -> bool {
        last_is_empty(self.as_str())
    }

    /// [`last_is_a_number`].
    pub fn last_is_a_number(&self) -> bool {
        last_is_a_number(self.as_str())
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> DomainSegments<'_> {
        DomainSegments(Cow::Borrowed(&self.0))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> DomainSegments<'static> {
        DomainSegments(self.0.into_owned().into())
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DomainSegments<'a> {
    type Error = InvalidDomainSegments;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, segments) = encode_domain_segments(value)?;

        Ok(Self(segments))
    }
}

impl<'a> From<DomainSegment<'a>> for DomainSegments<'a> {
    fn from(value: DomainSegment<'a>) -> Self {
        Self(value.0)
    }
}
