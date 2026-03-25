//! Labels stuff.

use crate::prelude::*;

impl BetterDomainHost<'_> {
    /// Get the labels.
    pub fn labels(&self) -> &str {
        &self.host[self.details.labels_range()]
    }

    /// Set the labels.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_labels(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        match value {
            Some(value) => {
                if value.len() + self.details.is_fqdn() as usize > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                let range = self.details.labels_range();
                self.host.to_mut().replace_range(range, value);
            },
            None => match self.details.fqddot_range() {
                Some(range) => self.host.retain_range(range),
                None => Err(CantBeEmpty)?
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}
