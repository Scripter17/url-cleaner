//! FQDN dot stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// If it has an FQDdot.
    pub fn has_fqddot(&self) -> bool {
        self.details.fq
    }

    /// If it's a fully qualified domain.
    pub fn is_fqdn(&self) -> bool {
        self.details.fq
    }



    /// The [`Range`] of the fqddot.
    fn fqddot_thing(&self) -> Option<Range<usize>> {
        match self.details.fq {
            false => None,
            true  => Some(self.len() - 1 .. self.len()),
        }
    }

    /// The FQDN dot as a [`str`].
    pub fn fqddot_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.fqddot_thing()?)})
    }



    /// Set the FQDN.
    /// # Errors
    /// If adding the FQDdot would make it too long, returns the error [`TooLong`].
    ///
    /// If removing the FQDdot would make the host empty, returns the error [`NonFqdnCantEndInEmpty`].
    pub fn set_fqdn(&mut self, value: bool) -> Result<bool, SetDomainError> {
        match (self.is_fqdn(), value) {
            (false, true ) if self.len() + 1 > u32::MAX as usize => Err(TooLong)?,
            // Assumes a trailing empty label is always the entire suffix.
            (true , false) if self.suffix_thing().is_empty() => Err(NonFqdnCantEndInEmpty)?,

            (false, false) => return Ok(false),
            (false, true ) => self.host.extend(["."]),
            (true , false) => self.host.retain_range(..self.suffix_after()),
            (true , true ) => return Ok(false),
        }

        self.details.fq = value;

        Ok(true)
    }
}
