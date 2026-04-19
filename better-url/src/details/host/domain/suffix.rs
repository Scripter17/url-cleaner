//! Suffix stuff.

use crate::prelude::*;

impl DomainDetails {
    /// The [`Range::start`] of the suffix.
    pub fn suffix_start(self) -> usize {
        self.ss as usize
    }

    /// The [`Range::end`] of the suffix.
    pub fn suffix_after(self) -> usize {
        self.sa as usize
    }

    /// The [`Range`] of the suffix.
    pub fn suffix_range(self) -> Range<usize> {
        self.suffix_start() .. self.suffix_after()
    }
}
