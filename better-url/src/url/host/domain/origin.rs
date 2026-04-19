//! Origin stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If the domain has an origin.
    pub fn has_domain_origin(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_origin)
    }

    /// The domain origin.
    pub fn domain_origin(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.origin_range()?])
    }

    /// The segments of the domain origin.
    pub fn domain_origin_segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.domain_origin().into_iter().flat_map(|x| x.split('.'))
    }

    /// A domain origin segment.
    pub fn domain_origin_segment(&self, index: isize) -> Option<&str> {
        self.domain_origin_segments().neg_nth(index)
    }

    /// [`DomainHost::set_origin`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_origin`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_origin(&mut self, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_origin(value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::set_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_origin_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.set_origin_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::replace_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::replace_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn replace_domain_origin_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.replace_origin_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }

    /// [`DomainHost::insert_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_origin_segment(&mut self, index: isize, value: &str) -> Result<(), SetDomainError> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_origin_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
