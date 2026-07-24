//! [`StrExt`].

use crate::prelude::*;

/// Extension trait for [`str`].
pub(crate) trait StrExt {
    /// Get the [`Range`] of a substring.
    fn my_substr_range(&self, substr: *const str) -> Range<usize>;

    /// Find the [`Range::start`] of the first instance of `substr`.
    fn find_start (&self, substr: &str) -> Option<usize>;

    /// Find the [`Range::end`] of the first instance of `substr`.
    fn find_after (&self, substr: &str) -> Option<usize>;

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

    fn find_start (&self, substr: &str) -> Option<usize> {memchr::memmem::find (self.as_bytes(), substr.as_bytes())                          }
    fn find_after (&self, substr: &str) -> Option<usize> {memchr::memmem::find (self.as_bytes(), substr.as_bytes()).map(|x| x + substr.len())}
    fn rfind_start(&self, substr: &str) -> Option<usize> {memchr::memmem::rfind(self.as_bytes(), substr.as_bytes())                          }
    fn rfind_after(&self, substr: &str) -> Option<usize> {memchr::memmem::rfind(self.as_bytes(), substr.as_bytes()).map(|x| x + substr.len())}

    fn addr(&self) -> usize {
        (self as *const str).addr()
    }
}
