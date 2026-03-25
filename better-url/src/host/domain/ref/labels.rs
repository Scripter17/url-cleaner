//! Labels stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// Get the labels.
    pub fn labels(self) -> &'a str {
        &self.host[self.details.labels_range()]
    }
}
