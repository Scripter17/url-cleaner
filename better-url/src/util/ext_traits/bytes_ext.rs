//! [`BytesExt`].

/// An extension trait for [`[u8]`].
pub(crate) trait BytesExt {
    /// The address of the slice.
    fn addr(&self) -> usize;

    /// The index of the first occurence of `b`.
    fn memchr(&self, b: u8) -> Option<usize>;

    /// The index of the last occurence of `b`.
    fn memrchr(&self, b: u8) -> Option<usize>;

    /// The index of the first occurence of any byte in `bs`. 
    fn memchrn<const N: usize>(&self, bs: [u8; N]) -> Option<usize>;
}

impl BytesExt for [u8] {
    fn addr(&self) -> usize {
        self.as_ptr().addr()
    }

    fn memchr (&self, b: u8) -> Option<usize> {
        unsafe {
            let a = libc::memchr(
                self as *const [u8] as *const libc::c_void,
                b as i32,
                self.len()
            );

            match a.addr() {
                0 => None,
                x => Some(x - self.as_ptr().addr())
            }
        }
    }

    fn memrchr(&self, b: u8) -> Option<usize> {
        #[cfg(target_os = "linux")]
        unsafe {
            let a = libc::memrchr(
                self as *const [u8] as *const libc::c_void,
                b as i32,
                self.len()
            );

            match a.addr() {
                0 => None,
                x => Some(x - self.as_ptr().addr())
            }
        }
        #[cfg(not(target_os = "linux"))]
        self.iter().rposition(|&byte| byte == b)
    }

    fn memchrn<const N: usize>(&self, bs: [u8; N]) -> Option<usize> {
        let mut ret = self.len();

        for b in bs {
            unsafe {
                let found = libc::memchr(
                    self as *const [u8] as *const libc::c_void,
                    b as i32,
                    ret,
                ).addr();

                if found != 0 {
                    ret = found - self.addr();
                }
            }
        }

        if ret == self.len() {
            None
        } else {
            Some(ret)
        }
    }
}
