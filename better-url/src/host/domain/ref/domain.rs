//! Domain stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// Get the segments.
    pub fn segments(self) -> impl DoubleEndedIterator<Item = &'a str> {
        self.labels().split('.')
    }

    /// Get the `index`th segment.
    pub fn segment(self, index: isize) -> Option<&'a str> {
        self.segments().neg_nth(index)
    }
}
