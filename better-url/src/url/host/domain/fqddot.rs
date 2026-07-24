//! Fqddot stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If it has a domain FQDdot.
    pub fn has_fqddot(&self) -> bool {
        self.domain_details().is_some_and(|x| x.fq)
    }

    /// If the host is a fully qualified domain name.
    pub fn is_fqdn(&self) -> bool {
        self.domain_details().is_some_and(|x| x.fq)
    }


    /// The [`Range`] of the domain FQDdot.
    fn domain_fqddot_thing(&self) -> Option<Range<usize>> {
        let ha = self.host_after    ()?;
        let dd = self.domain_details()?;

        match dd.fq {
            false => None,
            true  => Some(ha - 1 .. ha),
        }
    }

    /// The FQDDot as a [`str`].
    pub fn fqddot_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.domain_fqddot_thing()?)})
    }



    /// [`DomainHost::set_fqdn`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_fqdn`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_fqdn(&mut self, value: bool) -> Result<bool, SetHostError> {
        if self.domain_details().ok_or(NoDomain)?.fq != value {
            let mut domain = self.domain().ok_or(NoDomain)?;
            domain.set_fqdn(value)?;
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
