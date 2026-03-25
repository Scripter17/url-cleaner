//! FQDN dot stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`DomainDetails::has_fqddot`].
    pub fn has_fqddot(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_fqddot)
    }

    /// [`DomainDetails::is_fqdn`].
    pub fn is_fqdn(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::is_fqdn)
    }

    /// [`BetterRefDomainHost::fqddot`].
    pub fn fqddot(&self) -> Option<&str> {
        self.ref_domain()?.fqddot()
    }

    /// [`BetterDomainHost::set_fqdn`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_fqdn(&mut self, value: bool) -> Result<(), SetDomainError> {
        if self.is_fqdn() != value {
            let mut domain = self.domain().ok_or(NoDomain)?;
            domain.set_fqdn(value);
            self.set_better_host(Some(domain.into_owned()))?;
        }

        Ok(())
    }
}
