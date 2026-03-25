//! Prefix stuff.

use std::ops::Range;

use crate::prelude::*;

impl DomainDetails {
    /// Returns [`true`] if the domain has a prefix.
    pub fn has_prefix(self) -> bool {
        match self.ms {
            0 => false,
            _ => true
        }
    }

    /// The [`Range::start`] of the prefix.
    pub fn prefix_start(self) -> Option<usize> {
        match self.ms {
            0 => None,
            _ => Some(0)
        }
    }

    /// The [`Range::end`] of the prefix.
    pub fn prefix_after(self) -> Option<usize> {
        (self.ms as usize).checked_sub(1)
    }

    /// The [`Range`] of the prefix.
    pub fn prefix_range(self) -> Option<Range<usize>> {
        Some(self.prefix_start()? .. self.prefix_after()?)
    }

    /// The [`Range::start`] of the dot between the prefix and middle.
    pub fn predot_start(self) -> Option<usize> {
        match self.ms {
            0 => None,
            x => Some(x as usize - 1)
        }
    }

    /// The [`Range::end`] of the dot between the prefix and middle.
    pub fn predot_after(self) -> Option<usize> {
        match self.ms {
            0 => None,
            x => Some(x as usize)
        }
    }

    /// The [`Range`] of the dot between the prefix and middle.
    pub fn predot_range(self) -> Option<Range<usize>> {
        Some(self.predot_start()? .. self.predot_after()?)
    }
}
