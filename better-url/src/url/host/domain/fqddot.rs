//! Fqddot stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If the domain has a FQDN dot.
    pub fn has_fqddot(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_fqddot)
    }

    /// If the domain is an FQDN.
    pub fn is_fqdn(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::is_fqdn)
    }

    /// The FQDN dot.
    pub fn fqddot(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.fqddot_range()?])
    }

    /// [`DomainHost::set_fqdn`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_fqdn`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_fqdn(&mut self, value: bool) -> Result<(), SetDomainError> {
        if self.domain_details().ok_or(NoDomain)?.is_fqdn() == value {return Ok(());}

        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.is_fqdn() != value {
            domain.set_fqdn(value)?;
            self.set_host(Some(domain.into_owned()))?;
        }

        Ok(())
    }
}
