//! Getters.

use std::ops::{Bound, RangeBounds};

use crate::prelude::*;

impl BetterPathSegments<'_> {
    /// Get the `index`th [`RawPathSegment`].
    pub fn get(&self, index: isize) -> Option<RawPathSegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// Get a range of segments.
    pub fn get_range<B: RangeBounds<isize>>(&self, range: B) -> Option<BetterRefPathSegments<'_>> {
        let start = match range.start_bound() {
            Bound::Unbounded    => 0,
            Bound::Included(&x) => self.get(x    )?.0.addr() - self.0.addr(),
            Bound::Excluded(-1) => None?,
            Bound::Excluded(&x) => self.get(x + 1)?.0.addr() - self.0.addr(),
        };

        let end = match range.end_bound() {
            Bound::Unbounded    => self.0.len(),
            Bound::Included(&x) => self.get(x    )?.0.end_addr() - self.0.addr(),
            Bound::Excluded( 0) => None?,
            Bound::Excluded(&x) => self.get(x - 1)?.0.end_addr() - self.0.addr(),
        };

        self.0.get(start..end).map(Into::into)
    }
}
