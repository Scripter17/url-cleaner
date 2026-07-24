//! Removers.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// Either [`SpecialNotFilePath::pop`], [`FilePath::pop`], or [`NonSpecialPath::pop`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::pop`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::pop`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::pop`] returns an error, that error is returned.
    pub fn pop(&mut self) -> Result<(), SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.pop(),
            Self::File          (x) => x.pop(),
            Self::NonSpecial    (x) => x.pop(),
        }
    }

    /// Either [`SpecialNotFilePath::pop_if_empty`], [`FilePath::pop_if_empty`], or [`NonSpecialPath::pop_if_empty`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::pop_if_empty`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::pop_if_empty`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::pop_if_empty`] returns an error, that error is returned.
    pub fn pop_if_empty(&mut self) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.pop_if_empty(),
            Self::File          (x) => x.pop_if_empty(),
            Self::NonSpecial    (x) => x.pop_if_empty(),
        }
    }

    /// Either [`SpecialNotFilePath::remove`], [`FilePath::remove`], or [`NonSpecialPath::remove`].
    /// # Errors
    /// If the call to [`SpecialNotFilePath::remove`] returns an error, that error is returned.
    ///
    /// If the call to [`FilePath::remove`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialPath::remove`] returns an error, that error is returned.
    pub fn remove(&mut self, index: isize) -> Result<(), SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.remove(index),
            Self::File          (x) => x.remove(index),
            Self::NonSpecial    (x) => x.remove(index),
        }
    }
}
