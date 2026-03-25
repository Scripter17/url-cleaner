//! Prefix stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// [`DomainDetails::has_prefix`].
    pub fn has_prefix(self) -> bool {
        self.details.has_prefix()
    }
    
    /// Get the prefix.
    pub fn prefix(self) -> Option<&'a str> {
        self.details.prefix_range().map(|r| &self.host[r])
    }

    /// Get the prefix segments.
    pub fn prefix_segments(&self) -> impl DoubleEndedIterator<Item = &'a str> {
        self.prefix().into_iter().flat_map(|x| x.split('.'))
    }

    /// Get the `index`th prefix segment.
    pub fn prefix_segment(self, index: isize) -> Option<&'a str> {
        self.prefix_segments().neg_nth(index)
    }
}
