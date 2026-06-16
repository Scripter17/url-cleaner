//! Normal stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`DomainPartsDetails::has_normal`].
    pub fn has_domain_normal(&self) -> bool {
        self.domain_parts_details().is_some_and(DomainPartsDetails::has_normal)
    }



    /// The domain normal as a [`str`].
    pub fn domain_normal_str(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_parts_details()?.normal_range()])
    }

    /// The domain normal's [`BidiDetailsIter`].
    pub fn domain_normal_bidi_details(&self) -> Option<BidiDetailsIter<'_>> {
        Some(self.domain_details()?.normal_bidi_details())
    }

    /// The domain normal as a [`DomainSegments`].
    pub fn domain_normal(&self) -> Option<DomainSegments<'_>> {
        Some(DomainSegments {
            segments    : self.domain_normal_str         ()?.into(),
            bidi_details: self.domain_normal_bidi_details()?.into()
        })
    }



    /// The domain normal segments as [`str`]s.
    pub fn domain_normal_segment_strs(&self) -> Option<std::str::Split<'_, char>> {
        Some(self.domain_normal_str()?.split('.'))
    }

    /// The domain normal segments as [`DomainSegment`]s.
    pub fn domain_normal_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        Some(DomainSegmentsIter {
            segments    : self.domain_normal_segment_strs()?,
            bidi_details: self.domain_normal_bidi_details()?,
        })
    }



    /// The `index`th domain normal segment as a [`str`].
    pub fn domain_normal_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_normal_segment_strs()?.neg_nth(index)
    }

    /// The `index`th domain normal segment's [`BidiDetail`].
    pub fn domain_normal_segment_bidi_detail(&self, index: isize) -> Option<BidiDetail> {
        self.domain_normal_bidi_details()?.neg_nth(index)
    }

    /// The `index`th domain normal segment as a [`DomainSegment`].
    pub fn domain_normal_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_normal_segments()?.neg_nth(index)
    }



    /// The range of the domain normal segments as a [`str`].
    pub fn domain_normal_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        segments_range_thing(self.domain_normal_str()?, '.', range)
    }

    /// The range of the domain normal [`BidiDetailsIter`].
    pub fn domain_normal_range_bidi_details<B: RangeBounds<isize>>(&self, range: B) -> Option<BidiDetailsIter<'_>> {
        self.domain_normal_bidi_details()?.subrange(range)
    }

    /// The range of the domain normal segments as a [`DomainSegments`].
    pub fn domain_normal_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments {
            segments    : self.domain_normal_range_str         (range)?.into(),
            bidi_details: self.domain_normal_range_bidi_details(range)?.into(),
        })
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
            self.set_host(Some(domain.into_owned()))?;
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
            self.set_host(Some(domain.into_owned()))?;
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
            self.set_host(Some(domain.into_owned()))?;
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
        self.set_host(Some(domain.into_owned()))?;

        Ok(())
    }
}
