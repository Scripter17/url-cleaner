//! [`FilePathSegmentsIter`].

use crate::prelude::*;

/// An [`Iterator`] over the [`FilePathSegment`]s of a [`FilePathSegments`].
#[derive(Debug, Clone)]
pub struct FilePathSegmentsIter<'a>(pub(crate) SplitSlashes<'a>);

impl<'a> FilePathSegmentsIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.0.range(range)
    }

    /// The range of the remainder as a [`FilePathSegments`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<FilePathSegments<'a>> {
        self.range_str(range).map(|x| unsafe {FilePathSegments::new_unchecked(x)})
    }

    /// The remaining [`FilePathSegments`].
    pub fn remainder(&self) -> Option<FilePathSegments<'a>> {
        Some(unsafe {FilePathSegments::new_unchecked(self.0.remainder()?)})
    }

    /// The [`SplitSlashes`].
    pub fn inner(&self) -> &SplitSlashes<'a> {
        &self.0
    }
}

impl<'a> IntoIterator for &'a FilePath<'_> {
    type IntoIter = FilePathSegmentsIter<'a>;
    type Item     = FilePathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        FilePathSegmentsIter(SplitSlashes(Some(unsafe {self.as_str().get_unchecked(1..)})))
    }
}

impl<'a> IntoIterator for &'a FilePathSegments<'_> {
    type IntoIter = FilePathSegmentsIter<'a>;
    type Item     = FilePathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        FilePathSegmentsIter(SplitSlashes(Some(self.as_str())))
    }
}

impl<'a> Iterator for FilePathSegmentsIter<'a> {
    type Item = FilePathSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| unsafe {FilePathSegment::new_unchecked(x)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|x| unsafe {FilePathSegment::new_unchecked(x)})
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> DoubleEndedIterator for FilePathSegmentsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| unsafe {FilePathSegment::new_unchecked(x)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|x| unsafe {FilePathSegment::new_unchecked(x)})
    }
}
