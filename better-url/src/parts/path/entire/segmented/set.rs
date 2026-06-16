//! Setters.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// Either [`SpecialNotFileSegmentedPath::push`], [`FileSegmentedPath::push`], or [`NonSpecialSegmentedPath::push`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::push`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::push`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::push`] returns an error, that error is returned.
    pub fn push<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, value: T) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.push(value),
            Self::File          (x) => x.push(value),
            Self::NonSpecial    (x) => x.push(value),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::prepend`], [`FileSegmentedPath::prepend`], or [`NonSpecialSegmentedPath::prepend`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::prepend`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::prepend`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::prepend`] returns an error, that error is returned.
    pub fn prepend<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, value: T) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.prepend(value),
            Self::File          (x) => x.prepend(value),
            Self::NonSpecial    (x) => x.prepend(value),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::set`], [`FileSegmentedPath::set`], or [`NonSpecialSegmentedPath::set`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::set`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::set`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::set`] returns an error, that error is returned.
    pub fn set<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.set(index, value),
            Self::File          (x) => x.set(index, value),
            Self::NonSpecial    (x) => x.set(index, value),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::insert`], [`FileSegmentedPath::insert`], or [`NonSpecialSegmentedPath::insert`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::insert`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::insert`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::insert`] returns an error, that error is returned.
    pub fn insert<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: T) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.insert(index, value),
            Self::File          (x) => x.insert(index, value),
            Self::NonSpecial    (x) => x.insert(index, value),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::set_range`], [`FileSegmentedPath::set_range`], or [`NonSpecialSegmentedPath::set_range`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::set_range`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::set_range`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::set_range`] returns an error, that error is returned.
    pub fn set_range<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>, I: IntoIterator<Item = T>, B: RangeBounds<isize>>(&mut self, range: B, iter: I) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.set_range(range, iter),
            Self::File          (x) => x.set_range(range, iter),
            Self::NonSpecial    (x) => x.set_range(range, iter),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::insert_segments`], [`FileSegmentedPath::insert_segments`], or [`NonSpecialSegmentedPath::insert_segments`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::insert_segments`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::insert_segments`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::insert_segments`] returns an error, that error is returned.
    pub fn insert_segments<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>, I: IntoIterator<Item = T>>(&mut self, index: isize, iter: I) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.insert_segments(index, iter),
            Self::File          (x) => x.insert_segments(index, iter),
            Self::NonSpecial    (x) => x.insert_segments(index, iter),
        }
    }
}
