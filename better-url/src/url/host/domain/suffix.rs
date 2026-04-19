//! Suffix stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The domain suffix.
    pub fn domain_suffix(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.suffix_range()])
    }

    /// The segments of the domain suffix.
    pub fn domain_suffix_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.domain_suffix().into_iter().flat_map(|x| x.split('.'))
    }

    /// A domain suffix segment.
    pub fn domain_suffix_segment(&self, index: isize) -> Option<&str> {
        self.domain_suffix_segments().neg_nth(index)
    }

    /// [`DomainHost::set_suffix`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_suffix`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_suffix(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_suffix(value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::set_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_suffix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_suffix_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::replace_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::replace_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn replace_domain_suffix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_suffix_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::insert_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_suffix_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_suffix_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
