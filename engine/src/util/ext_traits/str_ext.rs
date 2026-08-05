//! [`StrExt`].

use crate::prelude::*;

/// Extension trait for [`str`].
pub(crate) trait StrExt {
    /// Get the [`Range`] of a substring.
    fn my_substr_range(&self, substr: *const str) -> Range<usize>;

    /// [`[u8]::memchr`].
    fn memchr(&self, b: u8) -> Option<usize>;

    /// Find the [`Range::start`] of the first instance of `substr`.
    fn find_start(&self, substr: &str) -> Option<usize>;

    /// Find the [`Range::end`] of the first instance of `substr`.
    fn find_after(&self, substr: &str) -> Option<usize>;

    /// Find the [`Range::start`] of the last instance of `substr`.
    fn rfind_start(&self, substr: &str) -> Option<usize>;

    /// Find the [`Range::end`] of the last instance of `substr`.
    fn rfind_after(&self, substr: &str) -> Option<usize>;

    /// Get the address of the [`str`].
    fn addr(&self) -> usize;
}

impl StrExt for str {
    fn my_substr_range(&self, substr: *const str) -> Range<usize> {
        let start = substr.addr() - self.addr();
        let end   = start + (substr as *const [u8]).len();

        start..end
    }

    fn memchr(&self, b: u8) -> Option<usize> {self.as_bytes().memchr(b)}

    fn find_start (&self, substr: &str) -> Option<usize> {
        unsafe {
            let a = libc::memmem(
                self as *const str as *const libc::c_void,
                self.len(),
                substr as *const str as *const libc::c_void,
                substr.len(),
            ).addr();

            match a {
                0 => None,
                x => Some(x - self.addr())
            }
        }
    }

    fn rfind_start(&self, substr: &str) -> Option<usize> {
        // TODO: Use Libc.
        memchr::memmem::rfind(self.as_bytes(), substr.as_bytes())
    }

    fn find_after (&self, substr: &str) -> Option<usize> {self. find_start(substr).map(|x| x + substr.len())}
    fn rfind_after(&self, substr: &str) -> Option<usize> {self.rfind_start(substr).map(|x| x + substr.len())}

    fn addr(&self) -> usize {
        (self as *const str).addr()
    }
}
