//! FQDN dot stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainPartsDetails::has_fqddot`].
    pub fn has_fqddot(&self) -> bool {
        self.details.parts.has_fqddot()
    }

    /// [`DomainPartsDetails::is_fqdn`].
    pub fn is_fqdn(&self) -> bool {
        self.details.parts.is_fqdn()
    }



    /// The FQDN dot as a [`str`].
    pub fn fqddot_str(&self) -> Option<&str> {
        self.details.parts.fqddot_range().map(|r| &self.host[r])
    }



    /// Set the FQDN.
    /// # Errors
    /// If adding the FQDN would make it too long, returns the error [`TooLong`].
    pub fn set_fqdn(&mut self, value: bool) -> Result<bool, SetDomainError> {
        match (self.is_fqdn(), value) {
            (false, true ) if self.len() + 1 > u32::MAX as usize => Err(TooLong)?,

            (false, false) => return Ok(false),
            (false, true ) => self.host.to_mut().push('.'),
            (true , false) => self.host.retain_range(..self.details.parts.suffix_after()),
            (true , true ) => return Ok(false),
        }

        self.details.parts.fq = value;

        Ok(true)
    }
}
