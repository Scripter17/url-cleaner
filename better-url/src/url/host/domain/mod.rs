//! [`DomainHost`] stuff.

use crate::prelude::*;

mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod origin;
mod labels;
mod normal;

impl BetterUrl {
    /// If it has a domain.
    pub fn has_domain(&self) -> bool {
        self.domain_details().is_some()
    }



    /// The domain host as a [`str`].
    pub fn domain_str(&self) -> Option<&str> {
        self.domain_details().and(self.host_str())
    }

    /// Shorthand for [`Self::domain_labels_segment_strs`].
    pub fn domain_segment_strs(&self) -> Option<SplitDots<'_>> {
        self.domain_labels_segment_strs()
    }

    /// Shorthand for [`Self::domain_labels_segment_str`].
    pub fn domain_segment_str(&self, index: isize) -> Option<&str> {
        self.domain_labels_segment_str(index)
    }

    /// Shorthand for [`Self::domain_labels_range_str`].
    pub fn domain_range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        self.domain_labels_range_str(range)
    }



    /// The [`DomainHost`].
    pub fn domain(&self) -> Option<DomainHost<'_>> {
        self.host()?.domain()
    }

    /// Shorthand for [`Self::domain_labels_segments`].
    pub fn domain_segments(&self) -> Option<DomainSegmentsIter<'_>> {
        self.domain_labels_segments()
    }

    /// Shorthand for [`Self::domain_labels_segment`].
    pub fn domain_segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.domain_labels_segment(index)
    }

    /// Shorthand for [`Self::domain_labels_range`].
    pub fn domain_range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        self.domain_labels_range(range)
    }



    /// [`DomainHost::set_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_segment(index, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`DomainHost::insert_segment`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::insert_segment`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn insert_domain_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        domain.insert_segment(index, value)?;
        self.set_host(domain.into_owned())?;

        Ok(())
    }

    /// [`DomainHost::set_range`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_range(range, value)? {
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
