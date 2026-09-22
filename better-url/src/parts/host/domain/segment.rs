//! [`DomainSegment`].

use crate::prelude::*;

/// A single domain segment.
#[derive(Debug, Clone)]
pub struct DomainSegment<'a>(pub(crate) Cow<'a, str>);

impl<'a> DomainSegment<'a> {
    /// Make a new [`Self`] with zero validity checks.
    /// # Safety
    /// `value` must be a valid domain segment literal.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T) -> Self {
        Self(value.into())
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// [`unchecked_domain_segment_to_unicode`].
    pub fn to_unicode(self) -> Cow<'a, str> {
        let (_, value, _) = unchecked_domain_segment_to_unicode(self.0);

        value
    }

    /// If it [`is_a_number`].
    pub fn is_a_number(&self) -> bool {
        is_a_number(&self.0)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> DomainSegment<'_> {
        DomainSegment(Cow::Borrowed(&self.0))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> DomainSegment<'static> {
        DomainSegment(self.0.into_owned().into())
    }
}



impl<'a> TryFrom<Cow<'a, [u8]>> for DomainSegment<'a> {
    type Error = InvalidDomainSegment;

    fn try_from(value: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        let (_, value) = domain_segment_bytes_to_ascii(value)?;

        unsafe {
            Ok(Self::new_unchecked(value))
        }
    }
}
