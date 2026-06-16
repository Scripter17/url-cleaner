//! [`DomainSegmentsIter`].

use crate::prelude::*;

/// An [`Iterator`] over the [`DomainSegment`]s of a [`DomainSegments`].
#[derive(Debug, Clone)]
pub struct DomainSegmentsIter<'a> {
    /// The segments.
    pub(crate) segments: std::str::Split<'a, char>,
    /// The [`BidiDetailsIter`].
    pub(crate) bidi_details: BidiDetailsIter<'a>,
}

impl<'a> DomainSegmentsIter<'a> {
    /// Borrow the inner [`std::str::Split`] and [`BidiDetailsIter`].
    pub fn as_inner(&self) -> (&std::str::Split<'a, char>, &BidiDetailsIter<'a>) {
        (&self.segments, &self.bidi_details)
    }

    /// Unwrap into the inner [`std::str::Split`] and [`BidiDetailsIter`].
    pub fn into_inner(self) -> (std::str::Split<'a, char>, BidiDetailsIter<'a>) {
        (self.segments, self.bidi_details)
    }
}

impl<'a> TryFrom<DomainSegmentsIter<'a>> for DomainSegments<'a> {
    type Error = CantBeEmpty;

    fn try_from(mut value: DomainSegmentsIter<'a>) -> Result<Self, Self::Error> {
        // TODO: Replace with [`std::str::Split::remainder`].

        let first = value.segments.next().ok_or(CantBeEmpty)?;
        let last  = value.segments.next_back().unwrap_or(first);

        let addr = first.addr();
        let len = last.end_addr() - addr;

        // TODO: Check that this is sound.

        let slice = unsafe {std::slice::from_raw_parts(first.as_ptr(), len)};
        let segments = unsafe {str::from_utf8_unchecked(slice)}.into();

        Ok(Self {
            segments,
            bidi_details: value.bidi_details.into(),
        })
    }
}

impl<'a> IntoIterator for &'a DomainSegments<'_> {
    type IntoIter = DomainSegmentsIter<'a>;
    type Item = DomainSegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DomainSegmentsIter {
            segments    : self.segments.split('.'),
            bidi_details: self.bidi_details.iter(),
        }
    }
}

impl<'a> Iterator for DomainSegmentsIter<'a> {
    type Item = DomainSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(DomainSegment {
            segment    : self.segments    .next()?.into(),
            bidi_detail: self.bidi_details.next()?,
        })
    }
}

impl<'a> DoubleEndedIterator for DomainSegmentsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(DomainSegment {
            segment    : self.segments    .next_back()?.into(),
            bidi_detail: self.bidi_details.next_back()?,
        })
    }
}

impl ExactSizeIterator for DomainSegmentsIter<'_> {
    fn len(&self) -> usize {
        self.bidi_details.len()
    }
}
