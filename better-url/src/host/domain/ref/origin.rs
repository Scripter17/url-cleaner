//! Origin stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// [`DomainDetails::has_origin`].
    pub fn has_origin(self) -> bool {
        self.details.has_origin()
    }
    
    /// Get the origin.
    pub fn origin(self) -> Option<&'a str> {
        self.details.origin_range().map(|r| &self.host[r])
    }

    /// Get the origin segments.
    pub fn origin_segments(self) -> impl DoubleEndedIterator<Item = &'a str> {
        self.origin().into_iter().flat_map(|x| x.split('.'))
    }

    /// Get the `index`th origin segment.
    pub fn origin_segment(self, index: isize) -> Option<&'a str> {
        self.origin_segments().neg_nth(index)
    }
}
