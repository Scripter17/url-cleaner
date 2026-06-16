//! Normal stuff.

use crate::prelude::*;

impl DomainPartsDetails {
    /// If it has a normal.
    pub fn has_normal(self) -> bool {
        true
    }

    /// The [`Range::start`] of the normal.
    pub fn normal_start(self) -> usize {
        match self.wp {
            false => 0,
            true  => self.ms as usize
        }
    }

    /// The [`Range::end`] of the normal.
    pub fn normal_after(self) -> usize {
        self.sa as usize
    }

    /// The [`Range`] of the normal.
    pub fn normal_range(self) -> Range<usize> {
        self.normal_start() .. self.normal_after()
    }
}
