//! [`CowStrExt`].

use crate::prelude::*;

/// Extension trait for `Cow<'_, str>`.
pub(crate) trait CowStrExt {
    /// Retain a substring using pointer arithmetic and no allocations.
    fn retain_substr(&mut self, substr: *const str);

    /// Retain a subslice with no allocations.
    fn retain_range<B: RangeBounds<usize>>(&mut self, range: B);

    /// Replace a range, trying to not allocate.
    fn replace_range<B: RangeBounds<usize>>(&mut self, range: B, with: &str);

    /// Replace a substring.
    fn replace_substr(&mut self, substr: *const str, with: &str);
}

impl CowStrExt for Cow<'_, str> {
    fn retain_substr(&mut self, substr: *const str) {
        self.retain_range(self.my_substr_range(substr));
    }

    fn retain_range<B: RangeBounds<usize>>(&mut self, range: B) {
        match self {
            Cow::Owned(x) => {
                match range.end_bound() {
                    Bound::Unbounded => {},
                    Bound::Excluded(&y) => x.truncate(y),
                    Bound::Included(&y) => x.truncate(y + 1),
                }
                match range.start_bound() {
                    Bound::Unbounded => {},
                    Bound::Excluded(&y) => x.replace_range(..=y, ""),
                    Bound::Included(&y) => x.replace_range(..y, "")
                }
            },
            Cow::Borrowed(x) => *x = &x[(range.start_bound().cloned(), range.end_bound().cloned())]
        }
    }

    fn replace_range<B: RangeBounds<usize>>(&mut self, range: B, with: &str) {
        match (range.start_bound(), range.end_bound(), with) {
            (Bound::Included(0) | Bound::Unbounded, Bound::Excluded(x), "") => self.retain_range((Bound::Included(x), Bound::Unbounded)),
            (Bound::Included(0) | Bound::Unbounded, Bound::Included(x), "") => self.retain_range((Bound::Excluded(x), Bound::Unbounded)),
            (Bound::Included(x)                   , Bound::Unbounded  , "") => self.retain_range((Bound::Unbounded, Bound::Excluded(x))),
            (Bound::Excluded(x)                   , Bound::Unbounded  , "") => self.retain_range((Bound::Unbounded, Bound::Included(x))),
            _ => self.to_mut().replace_range(range, with)
        }
    }

    fn replace_substr(&mut self, substr: *const str, with: &str) {
        self.replace_range(self.my_substr_range(substr), with)
    }
}
