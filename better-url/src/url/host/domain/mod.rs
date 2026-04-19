//! Implementing domain stuff for [`BetterUrl`].

use crate::prelude::*;

mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;
mod normal;

impl BetterUrl {
    /// Get a [`DomainHost`].
    pub fn domain(&self) -> Option<DomainHost<'_>> {
        self.host()?.domain()
    }

    /// Get the domain as a [`str`].
    pub fn domain_str(&self) -> Option<&str> {
        self.domain_details().and(self.host_str())
    }

    /// [`DomainHost::segments`].
    pub fn domain_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.domain_str().into_iter().flat_map(|x| x.split('.'))
    }

    /// [`DomainHost::segment`].
    pub fn domain_segment(&self, index: isize) -> Option<&str> {
        self.domain_segments().neg_nth(index)
    }

    /// [`DomainHost::set_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::replace_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::replace_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn replace_domain_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::insert_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
