//! [`VecBytesExt`].

use crate::prelude::*;

/// Extension trait for `Vec<u8>`.
pub(crate) trait VecBytesExt {
    /// Replace `range` without bounds checks.
    unsafe fn replace_range_unchecked<B: RangeBounds<usize>>(&mut self, range: B, with: &[u8]);

    /// Replace `range` with multiple slices at once and without bounds checks.
    unsafe fn replace_range_with_unchecked<B: RangeBounds<usize>>(&mut self, range: B, with: &[&[u8]]);
}

impl VecBytesExt for Vec<u8> {
    unsafe fn replace_range_unchecked<B: RangeBounds<usize>>(&mut self, range: B, with: &[u8]) {
        let start = match range.start_bound() {
            Bound::Unbounded    => 0,
            Bound::Excluded(&x) => x + 1,
            Bound::Included(&x) => x,
        };

        let after = match range.end_bound() {
            Bound::Unbounded    => self.len(),
            Bound::Excluded(&x) => x,
            Bound::Included(&x) => x + 1,
        };

        let len = self.len();

        if with.len() > after - start {
            self.reserve(with.len() - (after - start));
        }

        unsafe {
            std::ptr::copy(self.as_ptr().add(after), self.as_mut_ptr().add(start + with.len()), len - after);

            std::ptr::copy_nonoverlapping(with.as_ptr(), self.as_mut_ptr().add(start), with.len());

            self.set_len(len - (after - start) + with.len());
        }
    }
    unsafe fn replace_range_with_unchecked<B: RangeBounds<usize>>(&mut self, range: B, with: &[&[u8]]) {
        let start = match range.start_bound() {
            Bound::Unbounded    => 0,
            Bound::Excluded(&x) => x + 1,
            Bound::Included(&x) => x,
        };

        let after = match range.end_bound() {
            Bound::Unbounded    => self.len(),
            Bound::Excluded(&x) => x,
            Bound::Included(&x) => x + 1,
        };

        let len = self.len();
        let amt = with.iter().map(|x| x.len()).sum::<usize>();

        if amt > after - start {
            self.reserve(amt - (after - start));
        }

        unsafe {
            std::ptr::copy(self.as_ptr().add(after), self.as_mut_ptr().add(start + amt), len - after);

            let mut idx = start;

            for x in with {
                std::ptr::copy_nonoverlapping(x.as_ptr(), self.as_mut_ptr().add(idx), x.len());
                idx += x.len();
            }

            self.set_len(len - (after - start) + amt);
        }
    }
}
