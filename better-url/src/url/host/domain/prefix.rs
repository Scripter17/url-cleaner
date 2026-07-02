//! Prefix stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`DomainDetails::has_prefix`].
    pub fn has_domain_prefix(&self) -> bool {
        self.domain_details().is_some_and(DomainDetails::has_prefix)
    }



    /// The domain prefix as a [`str`].
    pub fn domain_prefix_str(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_details()?.prefix_range()?])
    }

    /// The domain prefix as a [`DomainSegments`].
    pub fn domain_prefix(&self) -> Option<DomainSegments<'_>> {
        Some(DomainSegments(self.domain_prefix_str()?.into()))
    }



    /// The domain prefix segments as [`str`]s.
    pub fn domain_prefix_segment_strs(&self) -> Option<SplitDots<'_>> {
        Some(SplitDots(Some(self.domain_prefix_str()?)))
    }

    /// The domain prefix's [`DomainSegmentsIter`].
    pub fn domain_prefix_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        self.domain_prefix_segment_strs().map(DomainSegmentsIter)
    }



    /// The `index`th domain prefix segment as a [`str`].
    pub fn domain_prefix_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_prefix_segment_strs()?.neg_nth(index)
    }

    /// The `index`th domain prefix segment as a [`DomainSegment`].
    pub fn domain_prefix_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_prefix_segments()?.neg_nth(index)
    }



    /// The range of domain prefix segments as a [`str`].
    pub fn domain_prefix_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        domain_range_thing(self.domain_prefix_str()?, range)
    }

    /// The range of domain prefix segments as a [`DomainSegments`].
    pub fn domain_prefix_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments(self.domain_prefix_range_str(range)?.into()))
    }



    /// [`DomainHost::set_prefix`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_prefix`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_prefix<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_prefix(value)? {
            self.set_host(Some(domain.into_owned()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_prefix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_prefix_segment(index, value)? {
            self.set_host(Some(domain.into_owned()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_prefix_range`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_prefix_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_prefix_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_prefix_range(range, value)? {
            self.set_host(Some(domain.into_owned()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::insert_prefix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_prefix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_prefix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_prefix_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
