//! Normal stuff.

use crate::prelude::*;

impl DomainHost<'_> {
    /// [`DomainDetails::has_normal`].
    pub fn has_normal(&self) -> bool {
        self.details().has_normal()
    }



    /// The normal as a [`str`].
    pub fn normal_str(&self) -> &str {
        &self.host[self.details.normal_range()]
    }

    /// The normal as a [`DomainSegments`].
    pub fn normal(&self) -> DomainSegments<'_> {
        DomainSegments(self.normal_str().into())
    }



    /// The normal segments as [`str`]s.
    pub fn normal_segment_strs(&self) -> SplitDots<'_> {
        SplitDots(Some(self.normal_str()))
    }

    /// The normal segments as [`DomainSegment`]s.
    pub fn normal_segments(&self) -> DomainSegmentsIter<'_> {
        DomainSegmentsIter(self.normal_segment_strs())
    }



    /// The `index`th normal segment as a [`str`].
    pub fn normal_segment_str(&self, index: isize) -> Option<&str> {
        self.normal_segment_strs().neg_nth(index)
    }

    /// The `index`th normal segment as a [`DomainSegment`].
    pub fn normal_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.normal_segments().neg_nth(index)
    }



    /// The range of normal segments as a [`str`].
    pub fn normal_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        domain_range_thing(self.normal_str(), range)
    }

    /// The range of normal segments as a [`DomainSegments`].
    pub fn normal_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());

        Some(DomainSegments(self.normal_range_str(range)?.into()))
    }



    /// Set the normal.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_normal<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        match self.details.wp {
            true  => self.set_origin(value                    ),
            false => self.set_labels(value.ok_or(CantBeEmpty)?),
        }
    }

    /// Set the `index`th normal segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_normal_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        match self.details.wp {
            true  => self.set_origin_segment(index, value),
            false => self.set_labels_segment(index, value),
        }
    }

    /// Set the `range` normal segments.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn set_normal_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        match self.details.wp {
            true  => self.set_origin_range(range, value),
            false => self.set_labels_range(range, value),
        }
    }

    /// Insert a new `index`th normal segment.
    /// # Errors
    /// See [`Self`]'s documentation.
    pub fn insert_normal_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        match self.details.wp {
            true  => self.insert_origin_segment(index, value),
            false => self.insert_labels_segment(index, value),
        }
    }
}
