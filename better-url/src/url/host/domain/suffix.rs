//! Suffix stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`BetterRefDomainHost::suffix`].
    pub fn domain_suffix(&self) -> Option<&str> {
        Some(self.ref_domain()?.suffix())
    }

    /// [`BetterRefDomainHost::suffix_segment`].
    pub fn domain_suffix_segment(&self, index: isize) -> Option<&str> {
        self.ref_domain()?.suffix_segment(index)
    }

    /// [`BetterDomainHost::set_suffix`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::set_suffix`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_domain_suffix(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_suffix(value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::set_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::set_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_domain_suffix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_suffix_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::replace_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::replace_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn replace_domain_suffix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_suffix_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::insert_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::insert_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn insert_domain_suffix_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_suffix_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
