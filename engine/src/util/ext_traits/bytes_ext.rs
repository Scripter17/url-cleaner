//! [`BytesExt`].

/// An extension trait for [`[u8]`].
pub(crate) trait BytesExt {
    /// The index of the first occurrence of `b`.
    fn memchr(&self, b: u8) -> Option<usize>;
}

impl BytesExt for [u8] {
    fn memchr(&self, b: u8) -> Option<usize> {
        better_url::util::memchr(self, b)
    }
}
