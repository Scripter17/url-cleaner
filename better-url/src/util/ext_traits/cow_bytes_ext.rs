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
        match self {
            Cow::Owned   (x) => {x.splice(range, with.iter().copied());},
            Cow::Borrowed(x) => {
                let start = match range.start_bound() {
                    Bound::Unbounded    => 0,
                    Bound::Included(&x) => x,
                    Bound::Excluded(&x) => x + 1,
                };

                let after = match range.end_bound() {
                    Bound::Unbounded    => x.len(),
                    Bound::Included(&x) => x + 1,
                    Bound::Excluded(&x) => x,
                };

                let mut ret = Vec::with_capacity(x.len() - (after - start) + with.len());

                ret.extend_from_slice(&x[..start]);
                ret.extend_from_slice(with);
                ret.extend_from_slice(&x[after..]);

                *self = ret.into();
            }
        }
    }
}
