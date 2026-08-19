//! [`BytesExt`].

/// An extension trait for [`[u8]`].
pub(crate) trait BytesExt {
    /// The address of the slice.
    fn addr(&self) -> usize;

    /// The index of the first occurence of `b`.
    fn memchr(&self, b: u8) -> Option<usize>;

    /// The index of the first occurence of any byte in `bs`. 
    fn memchrn<const N: usize>(&self, bs: [u8; N]) -> Option<usize>;
}

impl BytesExt for [u8] {
    fn addr(&self) -> usize {
        self.as_ptr().addr()
    }

    fn memchr(&self, b: u8) -> Option<usize> {
        let found = unsafe {libc::memchr(
            self.as_ptr() as _,
            b as _,
            self.len(),
        )}.addr();

        if found == 0 {
            None
        } else {
            Some(found - self.addr())
        }
    }

    fn memchrn<const N: usize>(&self, bs: [u8; N]) -> Option<usize> {
        let mut ret = self.len();

        for b in bs {
            let found = unsafe {libc::memchr(
                self.as_ptr() as _,
                b as _,
                ret,
            )}.addr();

            if found != 0 {
                ret = found - self.addr();
            }
        }

        if ret == self.len() {
            None
        } else {
            Some(ret)
        }
    }
}
