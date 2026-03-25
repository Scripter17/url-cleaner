//! Middle stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// [`DomainDetails::has_middle`].
    pub fn has_middle(self) -> bool {
        self.details.has_middle()
    }

    /// Get the middle.
    pub fn middle(self) -> Option<&'a str> {
        self.details.middle_range().map(|r| &self.host[r])
    }
}
