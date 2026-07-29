//! [`QueryIter`].

use crate::prelude::*;

/// A [`DoubleEndedIterator`] of [`QuerySegment`]s.
#[derive(Debug, Clone)]
pub struct QueryIter<'a> {
    /// The [`SplitAmpersands`].
    pub(crate) iter: SplitAmpersands<'a>,
    /// The [`QueryType`].
    pub(crate) r#type: QueryType,
}

impl<'a> QueryIter<'a> {
    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.iter.range(range)
    }

    /// The range of the remainder as a [`Query`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<Query<'a>> {
        self.range_str(range).map(|x| unsafe {Query::new_unchecked(x, self.r#type)})
    }

    /// The remaining [`Query`].
    pub fn remainder(&self) -> Option<Query<'a>> {
        Some(unsafe {Query::new_unchecked(self.iter.remainder()?, self.r#type)})
    }

    /// The [`SplitAmpersands`].
    pub fn inner(&self) -> &SplitAmpersands<'a> {
        &self.iter
    }

    /// The [`QueryType`].
    pub fn r#type(&self) -> QueryType {
        self.r#type
    }
}

impl<'a> IntoIterator for &'a Query<'_> {
    type IntoIter = QueryIter   <'a>;
    type Item     = QuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter {
            iter: SplitAmpersands(Some(self.as_str())),
            r#type: self.r#type(),
        }
    }
}

impl<'a> IntoIterator for &'a MaybeQuery<'_> {
    type IntoIter = QueryIter   <'a>;
    type Item     = QuerySegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter {
            iter: SplitAmpersands(self.as_str()),
            r#type: self.r#type(),
        }
    }
}

impl<'a> From<SpecialQueryIter   <'a>> for QueryIter<'a> {fn from(value: SpecialQueryIter   <'a>) -> Self {Self {iter: value.0, r#type: QueryType::Special   }}}
impl<'a> From<NonSpecialQueryIter<'a>> for QueryIter<'a> {fn from(value: NonSpecialQueryIter<'a>) -> Self {Self {iter: value.0, r#type: QueryType::NonSpecial}}}

impl<'a> Iterator for QueryIter<'a> {
    type Item = QuerySegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| unsafe {QuerySegment::new_unchecked(x, self.r#type)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|x| unsafe {QuerySegment::new_unchecked(x, self.r#type)})
    }

    fn count(self) -> usize {
        self.iter.count()
    }
}

impl<'a> DoubleEndedIterator for QueryIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|x| unsafe {QuerySegment::new_unchecked(x, self.r#type)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|x| unsafe {QuerySegment::new_unchecked(x, self.r#type)})
    }
}
