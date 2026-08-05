//! Removers.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// Either [`FilePath::pop`], [`SpecialNotFilePath::pop`], or [`NonSpecialPath::pop`].
    /// # Errors
    /// If the call to [`FilePath::pop`] returns an error, that error is returned.
    ///
    /// If the call to [`SpecialNotFilePath::pop`] returns an error, that error is returned.
    pub fn pop(&mut self) -> Result<bool, SetPathError> {
        match self {
            Self::File          (x) =>    x.pop() ,
            Self::SpecialNotFile(x) =>    x.pop() ,
            Self::NonSpecial    (x) => Ok(x.pop()),
        }
    }

    /// Either [`FilePath::pop_if_empty`], [`SpecialNotFilePath::pop_if_empty`], or [`NonSpecialPath::pop_if_empty`].
    /// # Errors
    /// If the call to [`FilePath::pop_if_empty`] returns an error, that error is returned.
    ///
    /// If the call to [`SpecialNotFilePath::pop_if_empty`] returns an error, that error is returned.
    pub fn pop_if_empty(&mut self) -> Result<bool, SetPathError> {
        match self {
            Self::File          (x) =>    x.pop_if_empty() ,
            Self::SpecialNotFile(x) =>    x.pop_if_empty() ,
            Self::NonSpecial    (x) => Ok(x.pop_if_empty()),
        }
    }

    /// Either [`FilePath::remove`], [`SpecialNotFilePath::remove`], or [`NonSpecialPath::remove`].
    /// # Errors
    /// If the call to [`FilePath::remove`] returns an error, that error is returned.
    ///
    /// If the call to [`SpecialNotFilePath::remove`] returns an error, that error is returned.
    pub fn remove(&mut self, index: isize) -> Result<bool, SetPathError> {
        match self {
            Self::File          (x) =>    x.remove(index) ,
            Self::SpecialNotFile(x) =>    x.remove(index) ,
            Self::NonSpecial    (x) => Ok(x.remove(index)),
        }
    }
}
