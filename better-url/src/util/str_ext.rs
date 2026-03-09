//! [`StrExt`].

use std::ops::Range;

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
}

impl StrExt for str {
    fn my_substr_range(&self, substr: *const str) -> Range<usize> {
        let start = substr.addr() - (self as *const str).addr();
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
}
