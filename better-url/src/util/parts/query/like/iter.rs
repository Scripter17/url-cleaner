//! [`QueryLikeIter`].

use crate::prelude::*;

/// A [`DoubleEndedIterator`] of [`QueryLikeSegment`]s.
#[derive(Debug, Clone)]
pub struct QueryLikeIter<'a> {
    /** The [`SplitAmpersands`]. **/ iter  : SplitAmpersands<'a>,
    /** The [`QueryLikeType`].   **/ r#type: QueryLikeType,
}

impl<'a> QueryLikeIter<'a> {
    /// Create a new [`Self`] from a [`SplitAmpersands`] without validity checks.
    /// # Safety
    /// Every output of [`SplitAmpersands::next`] must be a valid input to [`QueryLikeSegment::new_unchecked`] with the specified [`QueryLikeType`].
    pub unsafe fn from_parts(iter: SplitAmpersands<'a>, r#type: QueryLikeType) -> Self {
        Self {iter, r#type}
    }

    /// Get the inner [`SplitAmpersands`] and [`QueryLikeType`].
    pub fn into_parts(self) -> (SplitAmpersands<'a>, QueryLikeType) {
        (self.iter, self.r#type)
    }

    /// The range of the remainder as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&'a str> {
        self.iter.range(range)
    }

    /// The range of the remainder as a [`QueryLike`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<QueryLike<'a>> {
        self.range_str(range).map(|x| unsafe {QueryLike::new_unchecked(x, self.r#type)})
    }

    /// The remaining [`str`].
    pub fn remainder_str(&self) -> Option<&'a str> {
        self.iter.remainder()
    }

    /// The remaining [`QueryLike`].
    pub fn remainder(&self) -> Option<QueryLike<'a>> {
        Some(unsafe {QueryLike::new_unchecked(self.remainder_str()?, self.r#type)})
    }

    /// The [`QueryLikeType`].
    pub fn r#type(&self) -> QueryLikeType {
        self.r#type
    }
}

impl<'a> From<QueryIter          <'a>> for QueryLikeIter<'a> {fn from(value: QueryIter          <'a>) -> Self {let (iter, r#type) = value.into_parts(); Self{iter, r#type: r#type.into()}}}
impl<'a> From<SpecialQueryIter   <'a>> for QueryLikeIter<'a> {fn from(value: SpecialQueryIter   <'a>) -> Self {Self {iter: value.into_inner()   , r#type: QueryLikeType::Query(QueryType::Special   )}}}
impl<'a> From<NonSpecialQueryIter<'a>> for QueryLikeIter<'a> {fn from(value: NonSpecialQueryIter<'a>) -> Self {Self {iter: value.into_inner()   , r#type: QueryLikeType::Query(QueryType::NonSpecial)}}}
impl<'a> From<FragmentQueryIter  <'a>> for QueryLikeIter<'a> {fn from(value: FragmentQueryIter  <'a>) -> Self {Self {iter: value.into_inner()   , r#type: QueryLikeType::Fragment                    }}}

impl<'a> IntoIterator for &'a QueryLike<'_> {
    type IntoIter = QueryLikeIter   <'a>;
    type Item     = QueryLikeSegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        QueryLikeIter {
            iter: SplitAmpersands(Some(self.as_str())),
            r#type: self.r#type(),
        }
    }
}

impl<'a> IntoIterator for &'a MaybeQueryLike<'_> {
    type IntoIter = QueryLikeIter   <'a>;
    type Item     = QueryLikeSegment<'a>;

    fn into_iter(self) -> Self::IntoIter {
        QueryLikeIter {
            iter: SplitAmpersands(self.as_str()),
            r#type: self.r#type(),
        }
    }
}

impl<'a> Iterator for QueryLikeIter<'a> {
    type Item = QueryLikeSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| unsafe {QueryLikeSegment::new_unchecked(x, self.r#type)})
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|x| unsafe {QueryLikeSegment::new_unchecked(x, self.r#type)})
    }

    fn count(self) -> usize {
        self.iter.count()
    }
}

impl<'a> DoubleEndedIterator for QueryLikeIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|x| unsafe {QueryLikeSegment::new_unchecked(x, self.r#type)})
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|x| unsafe {QueryLikeSegment::new_unchecked(x, self.r#type)})
    }
}
