//! Origin stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`DomainPartsDetails::has_origin`].
    pub fn has_domain_origin(&self) -> bool {
        self.domain_parts_details().is_some_and(DomainPartsDetails::has_origin)
    }



    /// The domain origin as a [`str`].
    pub fn domain_origin_str(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_parts_details()?.origin_range()?])
    }

    /// The domain origin's [`BidiDetailsIter`].
    pub fn domain_origin_bidi_details(&self) -> Option<BidiDetailsIter<'_>> {
        self.domain_details()?.origin_bidi_details()
    }

    /// The domain origin as a [`DomainSegments`].
    pub fn domain_origin(&self) -> Option<DomainSegments<'_>> {
        Some(DomainSegments {
            segments    : self.domain_origin_str         ()?.into(),
            bidi_details: self.domain_origin_bidi_details()?.into(),
        })
    }



    /// The domain origin segments as [`str`]s.
    pub fn domain_origin_segment_strs(&self) -> Option<std::str::Split<'_, char>> {
        Some(self.domain_origin_str()?.split('.'))
    }

    /// The domain origin segments as [`DomainSegment`]s.
    pub fn domain_origin_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        Some(DomainSegmentsIter {
            segments    : self.domain_origin_segment_strs()?,
            bidi_details: self.domain_origin_bidi_details()?,
        })
    }



    /// The `index`th domain origin segment as a [`str`].
    pub fn domain_origin_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_origin_segment_strs()?.neg_nth(index)
    }

    /// The `index`th domain origin segment's [`BidiDetail`].
    pub fn domain_origin_segment_bidi_detail(&self, index: isize) -> Option<BidiDetail> {
        self.domain_origin_bidi_details()?.neg_nth(index)
    }

    /// The `index`th domain origin segment as a [`DomainSegment`].
    pub fn domain_origin_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_origin_segments()?.neg_nth(index)
    }



    /// The range of the domain origin segments as a [`str`].
    pub fn domain_origin_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        segments_range_thing(self.domain_origin_str()?, '.', range)
    }

    /// The range of the domain origin's [`BidiDetailsIter`].
    pub fn domain_origin_range_bidi_details<B: RangeBounds<isize>>(&self, range: B) -> Option<BidiDetailsIter<'_>> {
        self.domain_origin_bidi_details()?.subrange(range)
    }

    /// The range of the domain origin segments as a [`DomainSegments`].
    pub fn domain_origin_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments {
            segments    : self.domain_origin_range_str         (range)?.into(),
            bidi_details: self.domain_origin_range_bidi_details(range)?.into(),
        })
    }



    /// [`DomainHost::set_origin`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_origin`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_origin<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_origin(value)? {
            self.set_host(Some(domain.into_owned()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_origin_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_origin_segment(index, value)? {
            self.set_host(Some(domain.into_owned()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::set_origin_range`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_origin_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_origin_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_origin_range(range, value)? {
            self.set_host(Some(domain.into_owned()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::insert_origin_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_origin_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_origin_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;
        domain.insert_origin_segment(index, value)?;
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
