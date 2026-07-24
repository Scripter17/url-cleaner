//! Setters.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// Either [`SpecialNotFilePath::push`], [`FilePath::push`], or [`NonSpecialPath::push`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::push`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::push`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::push`] returns an error, that error is returned.
    pub fn push<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>>(&mut self, value: T) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.push(value),
            Self::File          (x) => x.push(value),
            Self::NonSpecial    (x) => x.push(value),
        }
    }

    /// Either [`SpecialNotFilePath::prepend`], [`FilePath::prepend`], or [`NonSpecialPath::prepend`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::prepend`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::prepend`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::prepend`] returns an error, that error is returned.
    pub fn prepend<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>>(&mut self, value: T) -> bool {
        match self {
            Self::SpecialNotFile(x) => x.prepend(value),
            Self::File          (x) => x.prepend(value),
            Self::NonSpecial    (x) => x.prepend(value),
        }
    }

    /// Either [`SpecialNotFilePath::set`], [`FilePath::set`], or [`NonSpecialPath::set`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::set`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::set`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::set`] returns an error, that error is returned.
    pub fn set<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.set(index, value),
            Self::File          (x) => x.set(index, value),
            Self::NonSpecial    (x) => x.set(index, value),
        }
    }

    /// Either [`SpecialNotFilePath::insert`], [`FilePath::insert`], or [`NonSpecialPath::insert`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::insert`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::insert`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::insert`] returns an error, that error is returned.
    pub fn insert<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>>(&mut self, index: isize, value: T) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.insert(index, value),
            Self::File          (x) => x.insert(index, value),
            Self::NonSpecial    (x) => x.insert(index, value),
        }
    }

    /// Either [`SpecialNotFilePath::set_range`], [`FilePath::set_range`], or [`NonSpecialPath::set_range`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::set_range`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::set_range`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::set_range`] returns an error, that error is returned.
    pub fn set_range<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.set_range(range, value),
            Self::File          (x) => x.set_range(range, value),
            Self::NonSpecial    (x) => x.set_range(range, value),
        }
    }
}
