//! Suffix stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// Get the suffix.
    pub fn suffix(self) -> &'a str {
        &self.host[self.details.suffix_range()]
    }

    /// Get the suffix segments.
    pub fn suffix_segments(self) -> impl DoubleEndedIterator<Item = &'a str> {
        self.suffix().split('.')
    }

    /// Get the `index`th suffix segment.
    pub fn suffix_segment(self, index: isize) -> Option<&'a str> {
        self.suffix_segments().neg_nth(index)
    }
}
