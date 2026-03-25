//! Reg domain stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`BetterRefDomainHost::has_origin`].
    pub fn has_domain_origin(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_origin)
    }

    /// [`BetterRefDomainHost::origin`].
    pub fn domain_origin(&self) -> Option<&str> {
        self.ref_domain()?.origin()
    }

    /// [`BetterRefDomainHost::origin_segment`].
    pub fn domain_origin_segment(&self, index: isize) -> Option<&str> {
        self.ref_domain()?.origin_segment(index)
    }

    /// [`BetterDomainHost::set_origin`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::set_origin`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_domain_origin(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_origin(value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::set_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::set_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_domain_origin_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_origin_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::replace_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::replace_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn replace_domain_origin_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_origin_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::insert_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::insert_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn insert_domain_origin_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_origin_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
