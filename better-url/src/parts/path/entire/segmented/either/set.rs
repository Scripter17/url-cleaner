//! Setters.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// Either [`FilePath::push`], [`SpecialNotFilePath::push`], or [`NonSpecialPath::push`].
    /// # Errors
    /// If [`FilePath::push`] returns an error, that error is returned.
    ///
    /// If [`SpecialNotFilePath::push`] returns an error, that error is returned.
    ///
    /// If [`NonSpecialPath::push`] returns an error, that error is returned.
    pub fn push<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, value: T) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.push(value),
            Self::File          (x) => x.push(value),
            Self::NonSpecial    (x) => x.push(value),
        }
    }

    /// Either [`FilePath::prepend`], [`SpecialNotFilePath::prepend`], or [`NonSpecialPath::prepend`].
    /// # Errors
    /// If [`FilePath::prepend`] returns an error, that error is returned.
    ///
    /// If [`SpecialNotFilePath::prepend`] returns an error, that error is returned.
    ///
    /// If [`NonSpecialPath::prepend`] returns an error, that error is returned.
    pub fn prepend<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, value: T) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.prepend(value),
            Self::File          (x) => x.prepend(value),
            Self::NonSpecial    (x) => x.prepend(value),
        }
    }

    /// Either [`FilePath::set`], [`SpecialNotFilePath::set`], or [`NonSpecialPath::set`].
    /// # Errors
    /// If [`FilePath::set`] returns an error, that error is returned.
    ///
    /// If [`SpecialNotFilePath::set`] returns an error, that error is returned.
    ///
    /// If [`NonSpecialPath::set`] returns an error, that error is returned.
    pub fn set<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.set(index, value),
            Self::File          (x) => x.set(index, value),
            Self::NonSpecial    (x) => x.set(index, value),
        }
    }

    /// Either [`FilePath::set_range`], [`SpecialNotFilePath::set_range`], or [`NonSpecialPath::set_range`].
    /// # Errors
    /// If [`FilePath::set_range`] returns an error, that error is returned.
    ///
    /// If [`SpecialNotFilePath::set_range`] returns an error, that error is returned.
    ///
    /// If [`NonSpecialPath::set_range`] returns an error, that error is returned.
    pub fn set_range<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.set_range(range, value),
            Self::File          (x) => x.set_range(range, value),
            Self::NonSpecial    (x) => x.set_range(range, value),
        }
    }

    /// Either [`FilePath::insert`], [`SpecialNotFilePath::insert`], or [`NonSpecialPath::insert`].
    /// # Errors
    /// If [`FilePath::insert`] returns an error, that error is returned.
    ///
    /// If [`SpecialNotFilePath::insert`] returns an error, that error is returned.
    ///
    /// If [`NonSpecialPath::insert`] returns an error, that error is returned.
    pub fn insert<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: T) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.insert(index, value),
            Self::File          (x) => x.insert(index, value),
            Self::NonSpecial    (x) => x.insert(index, value),
        }
    }
}
