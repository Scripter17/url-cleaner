//! [`BytesExt`].

use crate::prelude::*;

/// An extension trait for [`[u8]`].
pub(crate) trait BytesExt {
    /** [`memchr()`].  */ fn memchr (&self, b: u8) -> Option<usize>;
    /** [`memrchr()`]. */ fn memrchr(&self, b: u8) -> Option<usize>;
}

impl BytesExt for [u8] {
    fn memchr (&self, b: u8) -> Option<usize> {memchr (self, b)}
    fn memrchr(&self, b: u8) -> Option<usize> {memrchr(self, b)}
}
