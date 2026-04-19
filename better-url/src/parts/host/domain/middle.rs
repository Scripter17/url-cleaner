//! Middle stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainDetails::has_middle`].
    pub fn has_middle(&self) -> bool {
        self.details.has_middle()
    }

    /// Get the middle.
    pub fn middle(&self) -> Option<&str> {
        self.details.middle_range().map(|r| &self.host[r])
    }

    /// Set the middle.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_middle(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        match (self.middle(), value.map(|x| encode_domain(x.into())).as_deref()) {
            (None, None       ) => return Ok(()),
            (None, Some(value)) => {
                if self.len() + value.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                self.host.to_mut().insert_with(0, [value, "."]);
            },
            (Some(middle), None       ) => {
                let Range {start, end} = self.host.my_substr_range(middle);
                self.host.replace_range(start..=end, "");
            },
            (Some(middle), Some(value)) => {
                if middle == value {
                    return Ok(());
                }

                if self.len() - middle.len() + value.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                self.host.replace_substr(middle, value);
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}
