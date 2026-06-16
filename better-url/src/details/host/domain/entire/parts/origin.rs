//! Origin stuff.

use crate::prelude::*;

impl DomainPartsDetails {
    /// If it has an origin.
    pub fn has_origin(self) -> bool {
        self.ss != 0
    }

    /// The [`Range::start`] of the origin.
    pub fn origin_start(self) -> Option<usize> {
        match self.ss {
            0 => None,
            _ => Some(self.ms as usize)
        }
    }

    /// The [`Range::end`] of the origin.
    pub fn origin_after(self) -> Option<usize> {
        match self.ss {
            0 => None,
            _ => Some(self.sa as usize)
        }
    }

    /// The [`Range`] of the origin.
    pub fn origin_range(self) -> Option<Range<usize>> {
        Some(self.origin_start()? .. self.origin_after()?)
    }
}
