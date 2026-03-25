//! Implementing domain stuff for [`BetterUrl`].

use crate::prelude::*;

mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;

impl BetterUrl {
    /// Get a [`BetterDomainHost`].
    pub fn domain(&self) -> Option<BetterDomainHost<'_>> {
        self.host()?.domain()
    }

    /// Get a [`BetterRefDomainHost`].
    pub fn ref_domain(&self) -> Option<BetterRefDomainHost<'_>> {
        self.ref_host()?.domain()
    }

    /// Get the domain as a [`str`].
    pub fn domain_str(&self) -> Option<&str> {
        Some(self.ref_domain()?.as_str())
    }

    /// [`BetterRefDomainHost::segments`].
    pub fn domain_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.ref_domain().into_iter().flat_map(BetterRefDomainHost::segments)
    }

    /// [`BetterRefDomainHost::segment`].
    pub fn domain_segment(&self, index: isize) -> Option<&str> {
        self.ref_domain()?.segment(index)
    }

    /// [`BetterDomainHost::set_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::set_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn set_domain_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::replace_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::replace_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn replace_domain_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`BetterDomainHost::insert_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`BetterDomainHost::insert_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_better_host`] reutrns an error, that error is returned.
    pub fn insert_domain_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_segment(index, value)?;
        self.set_better_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
