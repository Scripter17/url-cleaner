//! [`SpecialQueryIter`].

use crate::prelude::*;

/// A [`DoubleEndedIterator`] of [`SpecialQuerySegment`]s.
#[derive(Debug, Clone)]
pub struct SpecialQueryIter<'a>(pub(crate) SplitAmpersands<'a>);

impl<'a> SpecialQueryIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.0.range(range)
    }

    /// The range of the remainder as a [`SpecialQuery`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<SpecialQuery<'a>> {
        self.range_str(range).map(|x| unsafe {SpecialQuery::new_unchecked(x)})
    }

    /// The remaining [`SpecialQuery`].
    pub fn remainder(&self) -> Option<SpecialQuery<'a>> {
        Some(unsafe {SpecialQuery::new_unchecked(self.0.remainder()?)})
    }

    /// The [`SplitAmpersands`].
    pub fn inner(&self) -> &SplitAmpersands<'a> {
        &self.0
    }
}

impl<'a> IntoIterator for &'a SpecialQuery<'_> {
    type IntoIter = SpecialQueryIter   <'a>;
    type Item     = SpecialQuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SpecialQueryIter(SplitAmpersands(Some(self.as_str())))
    }
}

impl<'a> IntoIterator for &'a MaybeSpecialQuery<'_> {
    type IntoIter = SpecialQueryIter   <'a>;
    type Item     = SpecialQuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SpecialQueryIter(SplitAmpersands(self.as_str()))
    }
}

impl<'a> Iterator for SpecialQueryIter<'a> {
    type Item = SpecialQuerySegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| unsafe {SpecialQuerySegment::new_unchecked(x)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|x| unsafe {SpecialQuerySegment::new_unchecked(x)})
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> DoubleEndedIterator for SpecialQueryIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| unsafe {SpecialQuerySegment::new_unchecked(x)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|x| unsafe {SpecialQuerySegment::new_unchecked(x)})
    }
}
