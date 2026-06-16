//! Middle stuff.

use crate::prelude::*;

impl DomainPartsDetails {
    /// If it has a middle.
    pub fn has_middle(self) -> bool {
        match self.ss {
            0 => false,
            _ => true
        }
    }

    /// The [`Range::start`] of the middle.
    pub fn middle_start(self) -> Option<usize> {
        match self.ss {
            0 => None,
            _ => Some(self.ms as usize)
        }
    }

    /// The [`Range::end`] of the middle.
    pub fn middle_after(self) -> Option<usize> {
        (self.ss as usize).checked_sub(1)
    }

    /// The [`Range`] of the middle.
    pub fn middle_range(self) -> Option<Range<usize>> {
        Some(self.middle_start()? .. self.middle_after()?)
    }

    /// The [`Range::start`] of the dot between the middle and suffix.
    pub fn middot_start(self) -> Option<usize> {
        match self.ss {
            0 => None,
            x => Some(x as usize - 1)
        }
    }

    /// The [`Range::end`] of the dot between the middle and suffix.
    pub fn middot_after(self) -> Option<usize> {
        match self.ss {
            0 => None,
            x => Some(x as usize)
        }
    }

    /// The [`Range`] of the dot between the middle and suffix.
    pub fn middot_range(self) -> Option<Range<usize>> {
        Some(self.middot_start()? .. self.middot_after()?)
    }
}
