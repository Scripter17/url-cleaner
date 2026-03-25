//! FQDN dot stuff.

use crate::prelude::*;

impl<'a> BetterRefDomainHost<'a> {
    /// [`DomainDetails::is_fqdn`].
    pub fn is_fqdn(self) -> bool {
        self.details.is_fqdn()
    }

    /// [`DomainDetails::has_fqddot`].
    pub fn has_fqddot(self) -> bool {
        self.details.has_fqddot()
    }

    /// Get the FQDN period.
    pub fn fqddot(self) -> Option<&'a str> {
        self.details.fqddot_range().map(|r| &self.host[r])
    }
}
