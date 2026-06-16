//! Removers.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// Either [`SpecialNotFileSegmentedPath::pop`], [`FileSegmentedPath::pop`], or [`NonSpecialSegmentedPath::pop`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::pop`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::pop`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::pop`] returns an error, that error is returned.
    pub fn pop(&mut self) -> Result<(), SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.pop(),
            Self::File          (x) => x.pop(),
            Self::NonSpecial    (x) => x.pop(),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::pop_if_empty`], [`FileSegmentedPath::pop_if_empty`], or [`NonSpecialSegmentedPath::pop_if_empty`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::pop_if_empty`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::pop_if_empty`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::pop_if_empty`] returns an error, that error is returned.
    pub fn pop_if_empty(&mut self) -> Result<bool, SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.pop_if_empty(),
            Self::File          (x) => x.pop_if_empty(),
            Self::NonSpecial    (x) => x.pop_if_empty(),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::remove`], [`FileSegmentedPath::remove`], or [`NonSpecialSegmentedPath::remove`].
    /// # Errors
    /// If the call to [`SpecialNotFileSegmentedPath::remove`] returns an error, that error is returned.
    ///
    /// If the call to [`FileSegmentedPath::remove`] returns an error, that error is returned.
    ///
    /// If the call to [`NonSpecialSegmentedPath::remove`] returns an error, that error is returned.
    pub fn remove(&mut self, index: isize) -> Result<(), SetPathError> {
        match self {
            Self::SpecialNotFile(x) => x.remove(index),
            Self::File          (x) => x.remove(index),
            Self::NonSpecial    (x) => x.remove(index),
        }
    }
}
