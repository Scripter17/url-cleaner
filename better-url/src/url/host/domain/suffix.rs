//! Suffix stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`DomainPartsDetails::has_suffix`].
    pub fn has_domain_suffix(&self) -> bool {
        self.domain_parts_details().is_some_and(DomainPartsDetails::has_suffix)
    }



    /// The domain suffix as a [`str`].
    pub fn domain_suffix_str(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_parts_details()?.suffix_range()])
    }

    /// The domain suffix's [`BidiDetailsIter`].
    pub fn domain_suffix_bidi_details(&self) -> Option<BidiDetailsIter<'_>> {
        Some(self.domain_details()?.suffix_bidi_details())
    }

    /// The domain suffix as a [`DomainSegments`].
    pub fn domain_suffix(&self) -> Option<DomainSegments<'_>> {
        Some(DomainSegments {
            segments    : self.domain_suffix_str         ()?.into(),
            bidi_details: self.domain_suffix_bidi_details()?.into(),
        })
    }



    /// The domain suffix segments as [`str`]s.
    pub fn domain_suffix_segment_strs(&self) -> Option<std::str::Split<'_, char>> {
        Some(self.domain_suffix_str()?.split('.'))
    }

    /// The domain suffix [`DomainSegmentsIter`].
    pub fn domain_suffix_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        Some(DomainSegmentsIter {
            segments    : self.domain_suffix_str()?.split('.'),
            bidi_details: self.domain_suffix_bidi_details()?,
        })
    }



    /// The `index`th domain suffix segment as a [`str`].
    pub fn domain_suffix_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_suffix_segment_strs()?.neg_nth(index)
    }

    /// The `index`th domain suffix segment's [`BidiDetail`].
    pub fn domain_suffix_segment_bidi_detail(&self, index: isize) -> Option<BidiDetail> {
        self.domain_suffix_bidi_details()?.neg_nth(index)
    }

    /// The `index`th domain suffix segment as a [`DomainSegment`].
    pub fn domain_suffix_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_suffix_segments()?.neg_nth(index)
    }



    /// The range of domain suffix segments as a [`str`].
    pub fn domain_suffix_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        segments_range_thing(self.domain_suffix_str()?, '.', range)
    }

    /// The range of domain suffix's [`BidiDetailsIter`].
    pub fn domain_suffix_range_bidi_details<B: RangeBounds<isize>>(&self, range: B) -> Option<BidiDetailsIter<'_>> {
        self.domain_details()?.suffix_bidi_details().subrange(range)
    }

    /// The range of domain suffix segments as a [`DomainSegments`].
    pub fn domain_suffix_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments {
            segments    : self.domain_suffix_range_str         (range)?.into(),
            bidi_details: self.domain_suffix_range_bidi_details(range)?.into(),
        })
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
            self.set_host(Some(domain.into_owned()))?;
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
            self.set_host(Some(domain.into_owned()))?;
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
            self.set_host(Some(domain.into_owned()))?;
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
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
