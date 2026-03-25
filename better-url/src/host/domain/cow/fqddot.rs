//! FQDN dot stuff.

use crate::prelude::*;

impl BetterDomainHost<'_> {
    /// [`DomainDetails::has_fqddot`].
    pub fn has_fqddot(&self) -> bool {
        self.details.has_fqddot()
    }

    /// [`DomainDetails::is_fqdn`].
    pub fn is_fqdn(&self) -> bool {
        self.details.is_fqdn()
    }

    /// Get the FQDN period.
    pub fn fqddot(&self) -> Option<&str> {
        self.details.fqddot_range().map(|r| &self.host[r])
    }

    /// Set the FQDN.
    pub fn set_fqdn(&mut self, value: bool) {
        match (self.is_fqdn(), value) {
            (false, false) => {},
            (false, true ) => self.host.to_mut().push('.'),
            (true , false) => self.host.retain_range(..self.details.suffix_after()),
            (true , true ) => {},
        }

        self.details.fq = value;
    }
}
