//! [`CowBytesExt`].

use crate::prelude::*;

/// Extension trait for `Cow<'_, [u8]>`.
pub(crate) trait CowBytesExt {
    /// Either [`Vec::set_len`] or [`slice::get_unchecked`].
    unsafe fn truncate_unchecked(&mut self, len: usize);

    /// Retain the range without bounds checks.
    unsafe fn retain_range_unchecked<B: RangeBounds<usize>>(&mut self, range: B);

    /// Replace the range.
    fn replace_range<B: RangeBounds<usize>>(&mut self, range: B, with: &[u8]);
}

impl CowBytesExt for Cow<'_, [u8]> {
    unsafe fn truncate_unchecked(&mut self, len: usize) {
        debug_assert!(len <= self.len());

        match self {
            Cow::Owned   (x) => unsafe {x.set_len(len)},
            Cow::Borrowed(x) => unsafe {*x = x.get_unchecked(..len)}
        }

        debug_assert_eq!(self.len(), len);
    }

    unsafe fn retain_range_unchecked<B: RangeBounds<usize>>(&mut self, range: B) {
        unsafe {
            match self {
                Cow::Owned(x) => {
                    match range.end_bound() {
                        Bound::Unbounded => {},
                        Bound::Excluded(&y) => x.set_len(y),
                        Bound::Included(&y) => x.set_len(y + 1),
                    }
                    match range.start_bound() {
                        Bound::Unbounded => {},
                        Bound::Excluded(&y) => {x.drain(..=y);},
                        Bound::Included(&y) => {x.drain(.. y);},
                    }
                },
                Cow::Borrowed(x) => *x = x.get_unchecked((range.start_bound().cloned(), range.end_bound().cloned()))
            }
        }
    }

    fn replace_range<B: RangeBounds<usize>>(&mut self, range: B, with: &[u8]) {
        let start = match range.start_bound() {
            Bound::Unbounded    => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };

        let after = match range.end_bound() {
            Bound::Unbounded    => self.len(),
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        };

        assert!(start <= after && after <= self.len());

        match self {
            Cow::Owned(x) if x.capacity() >= x.len() - (after - start) + with.len() => unsafe {
                std::ptr::copy(x.as_ptr().add(after), x.as_mut_ptr().add(start + with.len()), x.len() - after);

                std::ptr::copy_nonoverlapping(with.as_ptr(), x.as_mut_ptr().add(start), with.len());

                x.set_len(x.len() - (after - start) + with.len());
            },
            _ => {
                let mut ret = Vec::with_capacity(self.len() - (after - start) + with.len());

                unsafe {
                    std::ptr::copy_nonoverlapping(self.as_ptr()           , ret.as_mut_ptr()                        , start             );
                    std::ptr::copy_nonoverlapping(with.as_ptr()           , ret.as_mut_ptr().add(start             ), with.len()        );
                    std::ptr::copy_nonoverlapping(self.as_ptr().add(after), ret.as_mut_ptr().add(start + with.len()), self.len() - after);

                    ret.set_len(self.len() - (after - start) + with.len());
                }

                *self = ret.into();
            }
        }
    }
}
