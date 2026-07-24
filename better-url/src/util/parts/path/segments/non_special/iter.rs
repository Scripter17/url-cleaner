//! [`NonSpecialPathSegmentsIter`].

use crate::prelude::*;

/// An [`Iterator`] over the [`NonSpecialPathSegment`]s of a [`NonSpecialPathSegments`].
#[derive(Debug, Clone)]
pub struct NonSpecialPathSegmentsIter<'a>(pub(crate) SplitSlashes<'a>);

impl<'a> NonSpecialPathSegmentsIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.0.range(range)
    }

    /// The range of the remainder as a [`NonSpecialPathSegments`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<NonSpecialPathSegments<'a>> {
        self.range_str(range).map(|x| unsafe {NonSpecialPathSegments::new_unchecked(x)})
    }

    /// The remaining [`NonSpecialPathSegments`].
    pub fn remainder(&self) -> Option<NonSpecialPathSegments<'a>> {
        Some(unsafe {NonSpecialPathSegments::new_unchecked(self.0.remainder()?)})
    }

    /// The [`SplitSlashes`].
    pub fn inner(&self) -> &SplitSlashes<'a> {
        &self.0
    }
}

impl<'a> IntoIterator for &'a NonSpecialPath<'_> {
    type IntoIter = NonSpecialPathSegmentsIter<'a>;
    type Item     = NonSpecialPathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        NonSpecialPathSegmentsIter(SplitSlashes(self.as_str().strip_prefix('/')))
    }
}

impl<'a> IntoIterator for &'a NonSpecialPathSegments<'_> {
    type IntoIter = NonSpecialPathSegmentsIter<'a>;
    type Item     = NonSpecialPathSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        NonSpecialPathSegmentsIter(SplitSlashes(Some(self.as_str())))
    }
}

impl<'a> Iterator for NonSpecialPathSegmentsIter<'a> {
    type Item = NonSpecialPathSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| unsafe {NonSpecialPathSegment::new_unchecked(x)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|x| unsafe {NonSpecialPathSegment::new_unchecked(x)})
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> DoubleEndedIterator for NonSpecialPathSegmentsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| unsafe {NonSpecialPathSegment::new_unchecked(x)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|x| unsafe {NonSpecialPathSegment::new_unchecked(x)})
    }
}
