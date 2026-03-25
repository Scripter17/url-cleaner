//! Labels stuff.

use std::ops::Range;

use crate::prelude::*;

impl DomainDetails {
    /// The [`Range::start`] of the labels.
    pub fn labels_start(self) -> usize {
        0
    }

    /// The [`Range::end`] of the labels.
    pub fn labels_after(self) -> usize {
        self.sa as usize
    }

    /// The [`Range`] of the labels.
    pub fn labels_range(self) -> Range<usize> {
        self.labels_start() .. self.labels_after()
    }
}
