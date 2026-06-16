//! [`DomainSegment`].

use crate::prelude::*;

/// A single domain segment.
#[derive(Debug, Clone)]
pub struct DomainSegment<'a> {
    /// The segment.
    pub(crate) segment: Cow<'a, str>,
    /// The [`BidiDetail`].
    pub(crate) bidi_detail: BidiDetail,
}

impl<'a> DomainSegment<'a> {
    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.segment
    }

    /// The [`BidiDetail`].
    pub fn bidi_detail(&self) -> BidiDetail {
        self.bidi_detail
    }

    /// [`decode_normalized_domain_segment_unchecked`].
    pub fn decode(self) -> (Cow<'a, str>, BidiDetail) {
        let (_, value) = decode_normalized_domain_segment_unchecked(self.segment);
        (value, self.bidi_detail)
    }

    /// If it [`is_a_number`].
    pub fn is_a_number(&self) -> bool {
        is_a_number(&self.segment)
    }



    /// Turn into the inner [`Cow`].
    pub fn into_inner(self) -> (Cow<'a, str>, BidiDetail) {
        (self.segment, self.bidi_detail)
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> DomainSegment<'_> {
        DomainSegment {
            segment: Cow::Borrowed(&self.segment),
            bidi_detail: self.bidi_detail
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> DomainSegment<'static> {
        DomainSegment {
            segment: self.segment.into_owned().into(),
            bidi_detail: self.bidi_detail
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DomainSegment<'a> {
    type Error = InvalidDomainSegment;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, segment, bidi_detail) = encode_domain_segment(value)?;

        Ok(Self {segment, bidi_detail})
    }
}

impl<'a> TryFrom<DomainSegments<'a>> for DomainSegment<'a> {
    type Error = DomainSegments<'a>;

    fn try_from(value: DomainSegments<'a>) -> Result<Self, Self::Error> {
        match value.bidi_details.only() {
            Some(bidi_detail) => Ok(DomainSegment {
                segment: value.segments,
                bidi_detail,
            }),
            _ => Err(value)
        }
    }
}
