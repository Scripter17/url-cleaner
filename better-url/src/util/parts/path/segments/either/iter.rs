//! [`PathSegmentsIter`].

use crate::prelude::*;

/// An [`Iterator`] over the [`PathSegment`]s of a [`PathSegments`].
#[derive(Debug, Clone)]
pub struct PathSegmentsIter<'a> {
    /** The [`SplitSlashes`].      **/ pub(crate) iter  : SplitSlashes<'a>,
    /** The [`SegmentedPathType`]. **/ pub(crate) r#type: SegmentedPathType,
}

impl<'a> PathSegmentsIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.iter.range(range)
    }

    /// The range of the remainder as a [`PathSegments`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<PathSegments<'a>> {
        self.range_str(range).map(|x| unsafe {PathSegments::new_unchecked(x, self.r#type)})
    }

    /// The remaining [`PathSegments`].
    pub fn remainder(&self) -> Option<PathSegments<'a>> {
        Some(unsafe {PathSegments::new_unchecked(self.iter.remainder()?, self.r#type)})
    }

    /// The [`SplitSlashes`].
    pub fn inner(&self) -> &SplitSlashes<'a> {
        &self.iter
    }

    /// The [`SegmentedPathType`].
    pub fn r#type(&self) -> SegmentedPathType {
        self.r#type
    }
}

impl<'a> IntoIterator for &'a SegmentedPath<'_> {
    type IntoIter = PathSegmentsIter<'a>;
    type Item     = PathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        PathSegmentsIter {
            iter  : SplitSlashes(self.as_str().strip_prefix('/')),
            r#type: self.r#type(),
        }
    }
}

impl<'a> IntoIterator for &'a PathSegments<'_> {
    type IntoIter = PathSegmentsIter<'a>;
    type Item     = PathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        PathSegmentsIter {
            iter  : SplitSlashes(Some(self.as_str())),
            r#type: self.r#type(),
        }
    }
}

impl<'a> Iterator for PathSegmentsIter<'a> {
    type Item = PathSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(unsafe {PathSegment::new_unchecked(self.iter.next()?, self.r#type)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(unsafe {PathSegment::new_unchecked(self.iter.nth(n)?, self.r#type)})
    }

    fn count(self) -> usize {
        self.iter.count()
    }
}

impl<'a> DoubleEndedIterator for PathSegmentsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(unsafe {PathSegment::new_unchecked(self.iter.next_back()?, self.r#type)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        Some(unsafe {PathSegment::new_unchecked(self.iter.nth_back(n)?, self.r#type)})
    }
}

impl<'a> From<FilePathSegmentsIter          <'a>> for PathSegmentsIter<'a> {fn from(value: FilePathSegmentsIter          <'a>) -> Self {Self {iter: value.0, r#type: SegmentedPathType::File          }}}
impl<'a> From<SpecialNotFilePathSegmentsIter<'a>> for PathSegmentsIter<'a> {fn from(value: SpecialNotFilePathSegmentsIter<'a>) -> Self {Self {iter: value.0, r#type: SegmentedPathType::SpecialNotFile}}}
impl<'a> From<NonSpecialPathSegmentsIter    <'a>> for PathSegmentsIter<'a> {fn from(value: NonSpecialPathSegmentsIter    <'a>) -> Self {Self {iter: value.0, r#type: SegmentedPathType::NonSpecial    }}}
