//! Normal stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If it has a domain normal.
    pub fn has_domain_normal(&self) -> bool {
        self.domain_details().is_some()
    }



    /// The [`Range`] of the domain normal.
    fn domain_normal_thing(&self) -> Option<Range<usize>> {
        let hs = self.host_start    ()?;
        let ha = self.host_after    ()?;
        let dd = self.domain_details()?;

        Some(match dd.wp {
            false => hs                  .. ha - dd.fq as usize,
            true  => hs + dd.ms as usize .. ha - dd.fq as usize,
        })
    }

    /// The domain normal as a [`str`].
    pub fn domain_normal_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.domain_normal_thing()?)})
    }

    /// The domain normal as a [`DomainSegments`].
    pub fn domain_normal(&self) -> Option<DomainSegments<'_>> {
        Some(unsafe {DomainSegments::new_unchecked(self.domain_normal_str()?)})
    }



    /// The domain normal segments as [`str`]s.
    pub fn domain_normal_segment_strs(&self) -> Option<SplitDots<'_>> {
        Some(SplitDots(Some(self.domain_normal_str()?)))
    }

    /// The domain normal segments as [`DomainSegment`]s.
    pub fn domain_normal_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        self.domain_normal_segment_strs().map(DomainSegmentsIter)
    }



    /// The `index`th domain normal segment as a [`str`].
    pub fn domain_normal_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_normal_segment_strs()?.neg_nth(index)
    }

    /// The `index`th domain normal segment as a [`DomainSegment`].
    pub fn domain_normal_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_normal_segments()?.neg_nth(index)
    }



    /// The range of the domain normal segments as a [`str`].
    pub fn domain_normal_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        self.domain_normal_segments()?.range_str(range)
    }

    /// The range of the domain normal segments as a [`DomainSegments`].
    pub fn domain_normal_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        self.domain_normal_segments()?.range(range)
    }





    /// [`DomainHost::set_normal`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_normal`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_normal<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_normal(value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_normal_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_normal_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_normal_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_normal_segment(index, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_normal_range`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_normal_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_normal_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_normal_range(range, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::insert_normal_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_normal_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_normal_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_normal_segment(index, value)?;
        self.set_host(domain.into_owned())?;

        Ok(())
    }
}
