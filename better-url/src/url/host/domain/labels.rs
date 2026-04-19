//! Labels stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The domain labels.
    pub fn domain_labels(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.labels_range()])
    }

    /// [`DomainHost::set_labels`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_labels`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_labels(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_labels(value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
