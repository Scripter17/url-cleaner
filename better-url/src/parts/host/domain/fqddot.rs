//! FQDN dot stuff.

use crate::prelude::*;

impl DomainHost<'_> {
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
    /// # Errors
    /// If adding the FQDN would make it too long, returns the error [`TooLong`].
    pub fn set_fqdn(&mut self, value: bool) -> Result<(), SetDomainError> {
        match (self.is_fqdn(), value) {
            (false, false) => {},
            (false, true ) if self.len() + 1 > u32::MAX as usize => Err(TooLong)?,
            (false, true ) => self.host.to_mut().push('.'),
            (true , false) => self.host.retain_range(..self.details.suffix_after()),
            (true , true ) => {},
        }

        self.details.fq = value;

        Ok(())
    }
}
