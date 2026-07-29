//! [`FragmentQueryIter`].

use crate::prelude::*;

/// A [`DoubleEndedIterator`] of [`FragmentQuerySegment`]s.
#[derive(Debug, Clone)]
pub struct FragmentQueryIter<'a>(pub(crate) SplitAmpersands<'a>);

impl<'a> FragmentQueryIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.0.range(range)
    }

    /// The range of the remainder as a [`FragmentQuery`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<FragmentQuery<'a>> {
        self.range_str(range).map(|x| unsafe {FragmentQuery::new_unchecked(x)})
    }

    /// The remaining [`FragmentQuery`].
    pub fn remainder(&self) -> Option<FragmentQuery<'a>> {
        Some(unsafe {FragmentQuery::new_unchecked(self.0.remainder()?)})
    }

    /// The [`SplitAmpersands`].
    pub fn inner(&self) -> &SplitAmpersands<'a> {
        &self.0
    }
}

impl<'a> IntoIterator for &'a FragmentQuery<'_> {
    type IntoIter = FragmentQueryIter   <'a>;
    type Item     = FragmentQuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FragmentQueryIter(SplitAmpersands(Some(self.as_str())))
    }
}

impl<'a> IntoIterator for &'a MaybeFragmentQuery<'_> {
    type IntoIter = FragmentQueryIter   <'a>;
    type Item     = FragmentQuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FragmentQueryIter(SplitAmpersands(self.as_str()))
    }
}

impl<'a> Iterator for FragmentQueryIter<'a> {
    type Item = FragmentQuerySegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| unsafe {FragmentQuerySegment::new_unchecked(x)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|x| unsafe {FragmentQuerySegment::new_unchecked(x)})
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> DoubleEndedIterator for FragmentQueryIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| unsafe {FragmentQuerySegment::new_unchecked(x)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|x| unsafe {FragmentQuerySegment::new_unchecked(x)})
    }
}
