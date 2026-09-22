//! [`NonSpecialQueryIter`].

use crate::prelude::*;

/// A [`DoubleEndedIterator`] of [`NonSpecialQuerySegment`]s.
#[derive(Debug, Clone)]
pub struct NonSpecialQueryIter<'a>(pub(crate) SplitAmpersands<'a>);

impl<'a> NonSpecialQueryIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.0.range(range)
    }

    /// The range of the remainder as a [`NonSpecialQuery`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<NonSpecialQuery<'a>> {
        self.range_str(range).map(|x| unsafe {NonSpecialQuery::new_unchecked(x)})
    }

    /// The remaining [`str`].
    pub fn remainder_str(&self) -> Option<&'a str> {
        self.0.remainder()
    }

    /// The remaining [`NonSpecialQuery`].
    pub fn remainder(&self) -> Option<NonSpecialQuery<'a>> {
        Some(unsafe {NonSpecialQuery::new_unchecked(self.remainder_str()?)})
    }

    /// The [`SplitAmpersands`].
    pub fn inner(&self) -> &SplitAmpersands<'a> {
        &self.0
    }
}

impl<'a> IntoIterator for &'a NonSpecialQuery<'_> {
    type IntoIter = NonSpecialQueryIter   <'a>;
    type Item     = NonSpecialQuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NonSpecialQueryIter(SplitAmpersands(Some(self.as_str())))
    }
}

impl<'a> IntoIterator for &'a MaybeNonSpecialQuery<'_> {
    type IntoIter = NonSpecialQueryIter   <'a>;
    type Item     = NonSpecialQuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NonSpecialQueryIter(SplitAmpersands(self.as_str()))
    }
}

impl<'a> Iterator for NonSpecialQueryIter<'a> {
    type Item = NonSpecialQuerySegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| unsafe {NonSpecialQuerySegment::new_unchecked(x)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|x| unsafe {NonSpecialQuerySegment::new_unchecked(x)})
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

impl<'a> DoubleEndedIterator for NonSpecialQueryIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| unsafe {NonSpecialQuerySegment::new_unchecked(x)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).map(|x| unsafe {NonSpecialQuerySegment::new_unchecked(x)})
    }
}
