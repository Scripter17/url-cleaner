//! [`CowBytesExt`].

use crate::prelude::*;

/// Extension trait for `Cow<'_, [u8]>`.
pub(crate) trait CowBytesExt {
    /// Either [`Vec::set_len`] or [`slice::get_unchecked`].
    unsafe fn truncate_unchecked(&mut self, len: usize);
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
}
