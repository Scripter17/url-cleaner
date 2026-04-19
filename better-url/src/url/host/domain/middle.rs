//! Middle stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If the domain has a middle.
    pub fn has_domain_middle(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_middle)
    }

    /// The domain middle.
    pub fn domain_middle(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.middle_range()?])
    }

    /// [`DomainHost::set_middle`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_middle`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_middle(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_middle(value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
