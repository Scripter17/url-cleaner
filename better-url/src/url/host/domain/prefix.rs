//! Prefix stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If the domain has an prefix.
    pub fn has_domain_prefix(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_prefix)
    }

    /// The domain prefix.
    pub fn domain_prefix(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.prefix_range()?])
    }

    /// The segments of the domain prefix.
    pub fn domain_prefix_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.domain_prefix().into_iter().flat_map(|x| x.split('.'))
    }

    /// A domain prefix segment.
    pub fn domain_prefix_segment(&self, index: isize) -> Option<&str> {
        self.domain_prefix_segments().neg_nth(index)
    }

    /// [`DomainHost::set_prefix`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_prefix`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_prefix(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_prefix(value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::set_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_prefix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_prefix_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::replace_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::replace_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn replace_domain_prefix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_prefix_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::insert_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_prefix_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_prefix_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
