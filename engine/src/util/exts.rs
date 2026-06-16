//! Extension traits.

use crate::prelude::*;

/// Extension trait for [`str`].
pub(crate) trait StrExt {
    /// Get the [`Range`] of a substring.
    fn my_substr_range(&self, substr: *const str) -> Range<usize>;

    /// Get the address of the [`str`].
    fn addr(&self) -> usize;
}

impl StrExt for str {
    fn my_substr_range(&self, substr: *const str) -> Range<usize> {
        let start = substr.addr() - self.addr();
        let end   = start + (substr as *const [u8]).len();

        start..end
    }

    fn addr(&self) -> usize {
        (self as *const str).addr()
    }
}

/// Extension trait for [`Cow`] of [`str`].
pub(crate) trait CowStrExt {
    /// Retain the substring.
    fn retain_substr(&mut self, substr: *const str);

    /// Retain the range.
    fn retain_range<B: RangeBounds<usize>>(&mut self, range: B);

    /// Remove the range.
    fn remove_range<B: RangeBounds<usize>>(&mut self, range: B);

    /// Remove the substring.
    fn remove_substr(&mut self, substr: *const str);

    /// Replace the range.
    fn replace_range<B: RangeBounds<usize>>(&mut self, range: B, with: &str);
}

impl<'a> CowStrExt for Cow<'a, str> {
    fn retain_substr(&mut self, substr: *const str) {
        self.retain_range(self.my_substr_range(substr))
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

    fn remove_range<B: RangeBounds<usize>>(&mut self, range: B) {
        self.replace_range(range, "")
    }

    fn remove_substr(&mut self, substr: *const str) {
        self.remove_range(self.my_substr_range(substr))
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
}
