//! [`DomainSegments`].

use crate::prelude::*;

/// A sequence of domain segments.
#[derive(Debug, Clone)]
pub struct DomainSegments<'a>(Cow<'a, str>);

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

    /// [`uts46_domain_segments_to_unicode`].
    /// # Errors
    /// If [`uts46_domain_segments_to_unicode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fine.")]
    pub fn into_unicode(self) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
        uts46_domain_segments_to_unicode(self.into_inner())
    }

    /// [`Self::into_unicode`] without taking ownership.
    /// # Errors
    /// If [`uts46_domain_segments_to_unicode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fine.")]
    pub fn to_unicode(&self) -> Result<(Cow<'_, str>, Option<String>, BidiDetail), (Cow<'_, str>, Option<String>)> {
        uts46_domain_segments_to_unicode(self.as_str())
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



impl<'a> TryFrom<Cow<'a, [u8]>> for DomainSegments<'a> {
    type Error = InvalidDomainSegments;

    fn try_from(value: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        let (_, value) = make_domain_segments_bytes(value)?;

        unsafe {
            Ok(Self::new_unchecked(value))
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DomainSegments<'a> {
    type Error = InvalidDomainSegments;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, value) = make_domain_segments(value)?;

        unsafe {
            Ok(Self::new_unchecked(value))
        }
    }
}



impl<'a> From<DomainSegment<'a>> for DomainSegments<'a> {
    fn from(value: DomainSegment<'a>) -> Self {
        unsafe {
            Self::new_unchecked(value.into_inner())
        }
    }
}
