//! [`SpecialNotFilePathSegmentsIter`].

use crate::prelude::*;

/// An [`Iterator`] over the [`SpecialNotFilePathSegment`]s of a [`SpecialNotFilePathSegments`].
#[derive(Debug, Clone)]
pub struct SpecialNotFilePathSegmentsIter<'a>(pub(crate) SplitSlashes<'a>);

impl<'a> SpecialNotFilePathSegmentsIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.0.range(range)
    }

    /// The range of the remainder as a [`SpecialNotFilePathSegments`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<SpecialNotFilePathSegments<'a>> {
        self.range_str(range).map(|x| unsafe {SpecialNotFilePathSegments::new_unchecked(x)})
    }

    /// The remaining [`SpecialNotFilePathSegments`].
    pub fn remainder(&self) -> Option<SpecialNotFilePathSegments<'a>> {
        Some(unsafe {SpecialNotFilePathSegments::new_unchecked(self.0.remainder()?)})
    }

    /// The [`SplitSlashes`].
    pub fn inner(&self) -> &SplitSlashes<'a> {
        &self.0
    }
}

impl<'a> IntoIterator for &'a SpecialNotFilePath<'_> {
    type IntoIter = SpecialNotFilePathSegmentsIter<'a>;
    type Item     = SpecialNotFilePathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        SpecialNotFilePathSegmentsIter(SplitSlashes(Some(unsafe {self.as_str().get_unchecked(1..)})))
    }
}

impl<'a> IntoIterator for &'a SpecialNotFilePathSegments<'_> {
    type IntoIter = SpecialNotFilePathSegmentsIter<'a>;
    type Item     = SpecialNotFilePathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        SpecialNotFilePathSegmentsIter(SplitSlashes(Some(self.as_str())))
    }
}

impl<'a> Iterator for SpecialNotFilePathSegmentsIter<'a> {
    type Item = SpecialNotFilePathSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| unsafe {SpecialNotFilePathSegment::new_unchecked(x)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|x| unsafe {SpecialNotFilePathSegment::new_unchecked(x)})
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> DoubleEndedIterator for SpecialNotFilePathSegmentsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| unsafe {SpecialNotFilePathSegment::new_unchecked(x)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|x| unsafe {SpecialNotFilePathSegment::new_unchecked(x)})
    }
}
