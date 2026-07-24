//! [`CowStrExt`].

use crate::prelude::*;

/// Extension trait for [`Cow`] of [`str`].
pub(crate) trait CowStrExt {
    /// Retain the substring.
    fn retain_substr(&mut self, substr: *const str);

    /// Retain the range.
    fn retain_range<B: RangeBounds<usize>>(&mut self, range: B);

    /// Insert `string` at index `idx` without unnecessary allocations.
    fn insert_str(&mut self, idx: usize, string: &str);

    /// Append `string` without unnecessary allocations.
    fn push_str(&mut self, string: &str);
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
                    Bound::Excluded(&y) => x.truncate(y    ),
                    Bound::Included(&y) => x.truncate(y + 1),
                }
                match range.start_bound() {
                    Bound::Unbounded => {},
                    Bound::Excluded(&y) => x.replace_range(..=y, ""),
                    Bound::Included(&y) => x.replace_range(.. y, ""),
                }
            },
            Cow::Borrowed(x) => *x = &x[(range.start_bound().cloned(), range.end_bound().cloned())]
        }
    }

    fn insert_str(&mut self, idx: usize, string: &str) {
        match self {
            Cow::Owned(x) => x.insert_str(idx, string),
            Cow::Borrowed(x) => {
                let before = &x[..idx];
                let after = &x[idx..];
                let mut ret = String::with_capacity(x.len() + string.len());
                ret.extend([before, string, after]);
                *self = ret.into();
            }
        }
    }

    fn push_str(&mut self, string: &str) {
        match self {
            Cow::Owned(x) => x.push_str(string),
            Cow::Borrowed(x) => {
                let mut ret = String::with_capacity(x.len() + string.len());
                ret.extend([x, string]);
                *self = ret.into();
            }
        }
    }
}
