//! [`BidiDetailsIntoIter`].

use crate::prelude::*;

/// An owned [`Iterator`] over a [`BidiDetails`]'s [`BidiDetail`]s.
#[derive(Debug, Clone)]
pub struct BidiDetailsIntoIter {
    /// The [`BidiDetails`].
    pub(crate) details: BidiDetails,
    /// The [`Range`].
    pub(crate) range: Range<usize>,
}

impl ExactSizeIterator for BidiDetailsIntoIter {
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl Iterator for BidiDetailsIntoIter {
    type Item = BidiDetail;

    fn next(&mut self) -> Option<Self::Item> {
        self.details.uget(self.range.next()?)
    }
}

impl DoubleEndedIterator for BidiDetailsIntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.details.uget(self.range.next_back()?)
    }
}

impl IntoIterator for BidiDetails {
    type IntoIter = BidiDetailsIntoIter;
    type Item = BidiDetail;

    fn into_iter(self) -> Self::IntoIter {
        BidiDetailsIntoIter {
            range: 0..self.len(),
            details: self,
        }
    }
}
