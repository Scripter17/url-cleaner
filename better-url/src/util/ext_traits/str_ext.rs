//! [`StrExt`].

use crate::prelude::*;

/// Extension trait for [`str`].
pub(crate) trait StrExt {
    /// Get the range of a substring using pointer arithmetic.
    fn my_substr_range(&self, substr: *const str) -> Range<usize>;

    /// Get the substrings before and after the provided substring.
    fn split_around_substr<'a>(&'a self, substr: &str) -> (&'a str, &'a str);

    /// Get the address of the start.
    fn addr(&self) -> usize;

    /// [`Self::addr`] + [`str::len`].
    fn end_addr(&self) -> usize;

    /// [`str::trim_suffix`] but stable.
    fn my_trim_suffix(&self, suffix: &str) -> &str;

    /** [`[u8]::memchr`]. **/ fn memchr  (&self, b : u8                ) -> Option<usize>;
    /** [`[u8]::memchr2`]. **/ fn memchr2 (&self, b1: u8, b2: u8        ) -> Option<usize>;
    /** [`[u8]::memchr3`]. **/ fn memchr3 (&self, b1: u8, b2: u8, b3: u8) -> Option<usize>;

    /** [`[u8]::memrchr`]. **/ fn memrchr (&self, b : u8                ) -> Option<usize>;
    // /** [`[u8]::memrchr2`]. **/ fn memrchr2(&self, b1: u8, b2: u8        ) -> Option<usize>;
    // /** [`[u8]::memrchr3`]. **/ fn memrchr3(&self, b1: u8, b2: u8, b3: u8) -> Option<usize>;
}

impl StrExt for str {
    fn my_substr_range(&self, substr: *const str) -> Range<usize> {
        let start = substr.addr() - self.addr();
        let end   = start + (substr as *const [u8]).len(); // For some reason `*const str` doesn't have a `.len()`.

        start..end
    }

    fn split_around_substr<'a>(&'a self, substr: &str) -> (&'a str, &'a str) {
        let Range {start, end} = self.my_substr_range(substr);

        (&self[..start], &self[end..])
    }

    fn addr(&self) -> usize {
        (self as *const str).addr()
    }

    fn end_addr(&self) -> usize {
        self.addr() + self.len()
    }

    fn my_trim_suffix(&self, suffix: &str) -> &str {
        self.strip_suffix(suffix).unwrap_or(self)
    }

    fn memchr  (&self, b : u8                ) -> Option<usize> {self.as_bytes().memchr  (b         )}
    fn memchr2 (&self, b1: u8, b2: u8        ) -> Option<usize> {self.as_bytes().memchr2 (b1, b2    )}
    fn memchr3 (&self, b1: u8, b2: u8, b3: u8) -> Option<usize> {self.as_bytes().memchr3 (b1, b2, b3)}

    fn memrchr (&self, b : u8                ) -> Option<usize> {self.as_bytes().memrchr (b         )}
    // fn memrchr2(&self, b1: u8, b2: u8        ) -> Option<usize> {self.as_bytes().memrchr2(b1, b2    )}
    // fn memrchr3(&self, b1: u8, b2: u8, b3: u8) -> Option<usize> {self.as_bytes().memrchr3(b1, b2, b3)}
}
