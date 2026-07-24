//! Labels stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If it has a domain labels.
    pub fn has_domain_labels(&self) -> bool {
        self.domain_details().is_some()
    }



    /// The [`Range`] of the domain labels.
    fn domain_labels_thing(&self) -> Option<Range<usize>> {
        let hs = self.host_start    ()?;
        let ha = self.host_after    ()?;
        let dd = self.domain_details()?;

        Some(hs .. ha - dd.fq as usize)
    }

    /// The domain labels as a [`str`].
    pub fn domain_labels_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.domain_labels_thing()?)})
    }

    /// The domain labels as a [`DomainSegments`].
    pub fn domain_labels(&self) -> Option<DomainSegments<'_>> {
        Some(unsafe {DomainSegments::new_unchecked(self.domain_labels_str()?)})
    }



    /// The domain labels segments as [`str`]s.
    pub fn domain_labels_segment_strs(&self) -> Option<SplitDots<'_>> {
        Some(SplitDots(Some(self.domain_labels_str()?)))
    }

    /// The domain labels segments as a [`DomainSegmentsIter`].
    pub fn domain_labels_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        self.domain_labels_segment_strs().map(DomainSegmentsIter)
    }



    /// The `index`th domain labels segment as a [`str`].
    pub fn domain_labels_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_labels_segment_strs()?.neg_nth(index)
    }

    /// The `index`th domain labels segment as a [`DomainSegment`].
    pub fn domain_labels_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_labels_segments()?.neg_nth(index)
    }



    /// The range of the domain labels segments as a [`str`].
    pub fn domain_labels_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        self.domain_labels_segments()?.range_str(range)
    }

    /// The range of the domain labels segments as a [`DomainSegments`].
    pub fn domain_labels_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        self.domain_labels_segments()?.range(range)
    }



    /// [`DomainHost::set_labels`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_labels`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_labels<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: T) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_labels(value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_labels_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_labels_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_labels_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_labels_segment(index, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_labels_range`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_labels_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_labels_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_labels_range(range, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::insert_labels_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_labels_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_labels_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        domain.insert_labels_segment(index, value)?;
        self.set_host(domain.into_owned())?;

        Ok(())
    }
}
