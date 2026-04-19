//! Labels stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// Get the labels.
    pub fn labels(&self) -> &str {
        &self.host[self.details.labels_range()]
    }

    /// Set the labels.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_labels(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let labels = self.labels();

        match value.map(|x| encode_domain(x.into())).as_deref() {
            Some(value) => {
                if labels == value {
                    return Ok(());
                }

                if value.len() + self.details.is_fqdn() as usize > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                self.host.replace_substr(labels, value);
            },
            None => self.host.retain_range(self.details.fqddot_range().ok_or(CantBeEmpty)?)
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}
