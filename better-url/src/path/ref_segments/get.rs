//! Getters.

use std::ops::{Bound, RangeBounds};

use crate::prelude::*;

impl<'a> BetterRefPathSegments<'a> {
    /// Get the `index`th [`RawPathSegment`].
    pub fn get(&self, index: isize) -> Option<RawPathSegment<'a>> {
        self.iter().neg_nth(index)
    }

    /// Get a range of segments.
    /// # Examples
    /// ```
    /// use std::ops::Bound;
    /// use better_url::prelude::*;
    ///
    /// let path = BetterRefPathSegments::new("abc/def/ghi");
    ///
    /// assert_eq!(path.get_range((Bound::Excluded(0), Bound::Excluded(2))), Some("def".into()));
    /// assert_eq!(path.get_range((Bound::Excluded(0), Bound::Excluded(1))), None);
    /// ```
    pub fn get_range<B: RangeBounds<isize>>(&self, range: B) -> Option<BetterRefPathSegments<'a>> {
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
