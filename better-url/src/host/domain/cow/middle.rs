//! Middle stuff.

use std::ops::Range;

use crate::prelude::*;

impl BetterDomainHost<'_> {
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
        match (self.details.middle_range(), value) {
            (None, None       ) => return Ok(()),
            (None, Some(value)) => {
                if self.host.len() + value.len() + 1 > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                self.host.to_mut().insert_with(0, [value, "."]);
            },
            (Some(Range {start, end}), None       ) => self.host.replace_range(start..=end, ""),
            (Some(range)             , Some(value)) => {
                if self.host.len() - range.len() + value.len() > u32::MAX as usize {
                    Err(TooLong)?;
                }

                if value.bytes().any(invalid_domain_byte) {
                    Err(InvalidDomainByte)?;
                }

                self.host.replace_range(range, value);
            }
        }

        self.details = DomainDetails::parse_unchecked(&self.host);

        Ok(())
    }
}
