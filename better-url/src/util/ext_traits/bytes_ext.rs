//! [`BytesExt`].

use crate::prelude::*;

/// An extension trait for [`[u8]`].
pub(crate) trait BytesExt {
    /// The index of the first occurence of `b`.
    fn memchr(&self, b: u8) -> Option<usize>;

    /// The index of the last occurence of `b`.
    fn memrchr(&self, b: u8) -> Option<usize>;

    /// The index of the first occurence of any byte in `bs`. 
    fn memchrn<const N: usize>(&self, bs: [u8; N]) -> Option<usize>;
}

impl BytesExt for [u8] {
    fn memchr(&self, b: u8) -> Option<usize> {
        memchr(self, b)
    }

    fn memrchr(&self, b: u8) -> Option<usize> {
        memrchr(self, b)
    }

    fn memchrn<const N: usize>(&self, bs: [u8; N]) -> Option<usize> {
        memchrn(self, &bs)
    }
}
