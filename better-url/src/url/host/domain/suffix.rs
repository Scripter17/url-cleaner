//! Suffix stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If it has a domain suffix.
    pub fn has_domain_suffix(&self) -> bool {
        self.domain_details().is_some()
    }



    /// The [`Range`] of the domain suffix.
    fn domain_suffix_thing(&self) -> Option<Range<usize>> {
        let hs = self.host_start    ()?;
        let ha = self.host_after    ()?;
        let dd = self.domain_details()?;

        Some(hs + dd.ss as usize .. ha - dd.fq as usize)
    }

    /// The domain suffix as a [`str`].
    pub fn domain_suffix_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.domain_suffix_thing()?)})
    }

    /// The domain suffix as a [`DomainSegments`].
    pub fn domain_suffix(&self) -> Option<DomainSegments<'_>> {
        Some(unsafe {DomainSegments::new_unchecked(self.domain_suffix_str()?)})
    }



    /// The domain suffix segments as [`str`]s.
    pub fn domain_suffix_segment_strs(&self) -> Option<SplitDots<'_>> {
        Some(SplitDots(Some(self.domain_suffix_str()?)))
    }

    /// The domain suffix [`DomainSegmentsIter`].
    pub fn domain_suffix_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        self.domain_suffix_segment_strs().map(DomainSegmentsIter)
    }



    /// The `index`th domain suffix segment as a [`str`].
    pub fn domain_suffix_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_suffix_segment_strs()?.neg_nth(index)
    }

    /// The `index`th domain suffix segment as a [`DomainSegment`].
    pub fn domain_suffix_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_suffix_segments()?.neg_nth(index)
    }



    /// The range of domain suffix segments as a [`str`].
    pub fn domain_suffix_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        self.domain_suffix_segments()?.range_str(range)
    }

    /// The range of domain suffix segments as a [`DomainSegments`].
    pub fn domain_suffix_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        self.domain_suffix_segments()?.range(range)
    }



    /// [`DomainHost::set_suffix`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_suffix`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_suffix<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_suffix(value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_suffix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_suffix_segment(index, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_suffix_range`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_suffix_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_suffix_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_suffix_range(range, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::insert_suffix_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_suffix_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_suffix_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_suffix_segment(index, value)?;
        self.set_host(domain.into_owned())?;

        Ok(())
    }
}
