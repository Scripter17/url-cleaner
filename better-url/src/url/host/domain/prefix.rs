//! Prefix stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`BetterRefDomainHost::has_prefix`].
    pub fn has_domain_prefix(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_prefix)
    }

    /// [`BetterRefDomainHost::prefix`].
    pub fn domain_prefix(&self) -> Option<&str> {
        self.ref_domain()?.prefix()
    }

    /// [`BetterRefDomainHost::prefix_segment`].
    pub fn domain_prefix_segment(&self, index: isize) -> Option<&str> {
        self.ref_domain()?.prefix_segment(index)
    }

    /// [`BetterDomainHost::set_prefix`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::set_prefix`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_domain_prefix(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_prefix(value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::set_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::set_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_domain_prefix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_prefix_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::replace_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::replace_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn replace_domain_prefix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_prefix_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::insert_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::insert_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn insert_domain_prefix_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_prefix_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
