//! [`DomainSegmentsIter`].

use crate::prelude::*;

/// An [`Iterator`] over the [`DomainSegment`]s of a [`DomainSegments`].
#[derive(Debug, Clone)]
pub struct DomainSegmentsIter<'a>(pub(crate) SplitDots<'a>);

impl<'a> DomainSegmentsIter<'a> {
    /// The srange of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.0.range(range)
    }

    /// The srange of the remainder as a [`DomainSegments`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'a>> {
        self.range_str(range).map(|x| unsafe {DomainSegments::new_unchecked(x)})
    }

    /// The remaining [`DomainSegments`].
    pub fn remainder(&self) -> Option<DomainSegments<'a>> {
        self.0.0.map(|x| unsafe {DomainSegments::new_unchecked(x)})
    }
}

impl<'a> IntoIterator for &'a DomainSegments<'_> {
    type IntoIter = DomainSegmentsIter<'a>;
    type Item     = DomainSegment     <'a>;

    fn into_iter(self) -> Self::IntoIter {
        DomainSegmentsIter(SplitDots(Some(self.as_str())))
    }
}

impl<'a> Iterator for DomainSegmentsIter<'a> {
    type Item = DomainSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| unsafe {DomainSegment::new_unchecked(x)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|x| unsafe {DomainSegment::new_unchecked(x)})
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> DoubleEndedIterator for DomainSegmentsIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| unsafe {DomainSegment::new_unchecked(x)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|x| unsafe {DomainSegment::new_unchecked(x)})
    }
}
