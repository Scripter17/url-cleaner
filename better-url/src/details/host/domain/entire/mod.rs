//! [`DomainDetails`].

use crate::prelude::*;

mod parts;

pub use parts::*;

/// Details for a [`DomainHost`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DomainDetails {
    /// The [`DomainPartsDetails`].
    pub(crate) parts: DomainPartsDetails,
    /// The [`BidiDetails`].
    pub(crate) bidi : BidiDetails,
}

impl DomainDetails {
    /// The [`BidiDetails`].
    pub fn bidi(&self) -> &BidiDetails {
        &self.bidi
    }

    /// The [`DomainPartsDetails`].
    pub fn parts(&self) -> DomainPartsDetails {
        self.parts
    }



    /// The range of segment indices of the prefix.
    pub fn prefix_segments_urange(&self) -> Option<RangeTo<usize>> {
        Some(.. self.middle_segment_uindex()?)
    }

    /// The segment index of the middle.
    pub fn middle_segment_uindex(&self) -> Option<usize> {
        Some(self.bidi.len() - self.parts.mi?.get() as usize - 1)
    }

    /// The range of segment indices of the suffix.
    pub fn suffix_segments_urange(&self) -> RangeFrom<usize> {
        match self.middle_segment_uindex() {
            Some(x) => x + 1 ..,
            None    => 0 ..
        }
    }

    /// The range of segment indices of the labels.
    pub fn labels_segments_urange(&self) -> RangeFull {
        ..
    }

    /// The range of segment indices of the origin.
    pub fn origin_segments_urange(&self) -> Option<RangeFrom<usize>> {
        Some(self.middle_segment_uindex()?..)
    }

    /// The range of segment indices of the normal.
    pub fn normal_segments_urange(&self) -> RangeFrom<usize> {
        match self.parts.wp {
            true  => 1..,
            false => 0..,
        }
    }



    /// A [`BidiDetailsIter`] for the prefix.
    pub fn prefix_bidi_details(&self) -> Option<BidiDetailsIter<'_>> {
        self.bidi.urange(self.prefix_segments_urange()?)
    }

    /// The [`BidiDetail`] for the middle.
    pub fn middle_bidi_detail(&self) -> Option<BidiDetail> {
        self.bidi.uget(self.middle_segment_uindex()?)
    }

    /// A [`BidiDetailsIter`] for the suffix.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn suffix_bidi_details(&self) -> BidiDetailsIter<'_> {
        self.bidi.urange(self.suffix_segments_urange()).expect("To always have a suffix.")
    }

    /// A [`BidiDetailsIter`] for the labels.
    pub fn labels_bidi_details(&self) -> BidiDetailsIter<'_> {
        self.bidi.iter()
    }

    /// A [`BidiDetailsIter`] for the origin.
    pub fn origin_bidi_details(&self) -> Option<BidiDetailsIter<'_>> {
        self.bidi.urange(self.origin_segments_urange()?)
    }

    /// A [`BidiDetailsIter`] for the normal.
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn normal_bidi_details(&self) -> BidiDetailsIter<'_> {
        self.bidi.urange(self.normal_segments_urange()).expect("To always have a normal.")
    }



    /// The length of the domain.
    #[allow(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(&self) -> usize {
        self.parts.len()
    }
}

impl TryFrom<HostDetails> for DomainDetails {
    type Error = HostDetails;

    fn try_from(value: HostDetails) -> Result<Self, Self::Error> {
        match value {
            HostDetails::Domain(details) => Ok(details),
            details => Err(details)
        }
    }
}
