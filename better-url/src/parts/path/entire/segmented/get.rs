//! Getters.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// Either [`FilePath::iter_strs`], [`SpecialNotFilePath::iter_strs`], or [`NonSpecialPath::iter_strs`].
    pub fn iter_strs(&self) -> SplitSlashes<'_> {
        match self {
            Self::File          (x) => x.iter_strs(),
            Self::SpecialNotFile(x) => x.iter_strs(),
            Self::NonSpecial    (x) => x.iter_strs(),
        }
    }

    /// Either [`FilePath::get_str`], [`SpecialNotFilePath::get_str`], or [`NonSpecialPath::get_str`].
    pub fn get_str(&self, index: isize) -> Option<&str> {
        match self {
            Self::File          (x) => x.get_str(index),
            Self::SpecialNotFile(x) => x.get_str(index),
            Self::NonSpecial    (x) => x.get_str(index),
        }
    }

    /// Either [`FilePath::range_str`], [`SpecialNotFilePath::range_str`], or [`NonSpecialPath::range_str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        match self {
            Self::File          (x) => x.range_str(range),
            Self::SpecialNotFile(x) => x.range_str(range),
            Self::NonSpecial    (x) => x.range_str(range),
        }
    }



    /// Either [`FilePath::iter`], [`SpecialNotFilePath::iter`], or [`NonSpecialPath::iter`].
    pub fn iter(&self) -> PathSegmentsIter<'_> {
        match self {
            Self::File          (x) => x.iter().into(),
            Self::SpecialNotFile(x) => x.iter().into(),
            Self::NonSpecial    (x) => x.iter().into(),
        }
    }

    /// Either [`FilePath::get`], [`SpecialNotFilePath::get`], or [`NonSpecialPath::get`].
    pub fn get(&self, index: isize) -> Option<PathSegment<'_>> {
        match self {
            Self::File          (x) => x.get(index).map(Into::into),
            Self::SpecialNotFile(x) => x.get(index).map(Into::into),
            Self::NonSpecial    (x) => x.get(index).map(Into::into),
        }
    }

    /// Either [`FilePath::range`], [`SpecialNotFilePath::range`], or [`NonSpecialPath::range`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<PathSegments<'_>> {
        match self {
            Self::File          (x) => x.range(range).map(Into::into),
            Self::SpecialNotFile(x) => x.range(range).map(Into::into),
            Self::NonSpecial    (x) => x.range(range).map(Into::into),
        }
    }
}
