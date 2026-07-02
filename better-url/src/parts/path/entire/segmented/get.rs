//! Getters.

use crate::prelude::*;

impl SegmentedPath<'_> {
    /// The [`SchemeType`].
    fn scheme_type(&self) -> SchemeType {
        match self {
            Self::SpecialNotFile(_) => SchemeType::SpecialNotFile,
            Self::File          (_) => SchemeType::File,
            Self::NonSpecial    (_) => SchemeType::NonSpecial,
        }
    }

    /// The [`PathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = PathSegment<'_>> {
        let scheme_type = self.scheme_type();

        SplitSlashes(Some(&self.as_str()[1..])).map(move |x| {
            match scheme_type {
                SchemeType::SpecialNotFile => SpecialNotFilePathSegment(x.into()).into(),
                SchemeType::File           => FilePathSegment          (x.into()).into(),
                SchemeType::NonSpecial     => NonSpecialPathSegment    (x.into()).into(),
            }
        })
    }

    /// Either [`SpecialNotFileSegmentedPath::get`], [`FileSegmentedPath::get`], or [`NonSpecialSegmentedPath::get`].
    pub fn get(&self, index: isize) -> Option<PathSegment<'_>> {
        match self {
            Self::SpecialNotFile(x) => x.get(index).map(Into::into),
            Self::File          (x) => x.get(index).map(Into::into),
            Self::NonSpecial    (x) => x.get(index).map(Into::into),
        }
    }

    /// Either [`SpecialNotFileSegmentedPath::get_range`], [`FileSegmentedPath::get_range`] or [`NonSpecialSegmentedPath::get_range`].
    pub fn get_range<B: RangeBounds<isize>>(&self, range: B) -> Option<PathSegments<'_>> {
        match self {
            Self::SpecialNotFile(x) => x.get_range(range).map(Into::into),
            Self::File          (x) => x.get_range(range).map(Into::into),
            Self::NonSpecial    (x) => x.get_range(range).map(Into::into),
        }
    }
}
