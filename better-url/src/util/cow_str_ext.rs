//! [`CowStrExt`].

use std::borrow::Cow;
use std::ops::Range;

use crate::prelude::*;

/// Extension trait for `Cow<'_, str>`.
pub(crate) trait CowStrExt {
    /// Retain a substring using pointer arithmetic and no allocations.
    fn retain_substr(&mut self, substr: *const str);

    /// Retain a subslice with no allocations.
    fn retain_range(&mut self, range: Range<usize>);
}

impl CowStrExt for Cow<'_, str> {
    fn retain_substr(&mut self, substr: *const str) {
        self.retain_range(self.my_substr_range(substr));
    }

    fn retain_range(&mut self, range: Range<usize>) {
        match self {
            Cow::Owned   (x) => {x.truncate(range.end); x.drain(..range.start).for_each(drop);},
            Cow::Borrowed(x) => *x = &x[range]
        }
    }
}
