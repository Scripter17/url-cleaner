//! [`DomainSegments`].

use crate::prelude::*;

mod iter;

pub use iter::*;

/// A sequence of domain segments.
#[derive(Debug, Clone)]
pub struct DomainSegments<'a> {
    /// The segments.
    pub(crate) segments: Cow<'a, str>,
    /// The [`BidiDetails`].
    pub(crate) bidi_details: BidiDetails,
}

impl<'a> DomainSegments<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.segments
    }

    /// The [`BidiDetails`].
    pub fn bidi_details(&self) -> &BidiDetails {
        &self.bidi_details
    }

    /// [`decode_normalized_domain_segments_unchecked`].
    pub fn decode(self) -> (Cow<'a, str>, BidiDetails) {
        let (_, value) = decode_normalized_domain_segments_unchecked(self.segments);
        (value, self.bidi_details)
    }

    /// A [`DomainSegmentsIter`].
    pub fn iter(&self) -> DomainSegmentsIter<'_> {
        self.into_iter()
    }

    /// If it [`ends_in_a_number`].
    pub fn ends_in_a_number(&self) -> bool {
        ends_in_a_number(self.as_str())
    }

    /// If [`Self::last`] [`DomainSegment::is_empty`].
    pub fn last_is_empty(&self) -> bool {
        self.as_str().is_empty() || self.as_str().ends_with('.')
    }

    /// If [`last`] [`DomainSegment::is_a_number`].
    pub fn last_is_a_number(&self) -> bool {
        last_is_a_number(self.as_str().rsplit_once('.').map_or(self.as_str(), |(_, x)| x))
    }

    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> (Cow<'a, str>, BidiDetails) {
        (self.segments, self.bidi_details)
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> DomainSegments<'static> {
        DomainSegments {
            segments: self.segments.into_owned().into(),
            bidi_details: self.bidi_details.clone(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> DomainSegments<'_> {
        DomainSegments {
            segments: Cow::Borrowed(&self.segments),
            bidi_details: self.bidi_details.clone(),
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DomainSegments<'a> {
    type Error = InvalidDomainSegments;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, segments, bidi_details) = encode_domain_segments(value)?;

        Ok(Self {segments, bidi_details})
    }
}

impl<'a> From<DomainSegment<'a>> for DomainSegments<'a> {
    fn from(value: DomainSegment<'a>) -> Self {
        Self {
            segments: value.segment,
            bidi_details: value.bidi_detail.into()
        }
    }
}
