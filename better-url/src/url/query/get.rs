//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the query.
    fn query_start(&self) -> Option<usize> {
        Some(self.query_mark?.get() as usize + 1)
    }

    /// The [`Range::end`] of the query.
    fn query_after(&self) -> Option<usize> {
        Some(self.fragment_mark.map_or(self.len(), |x| x.get() as usize))
    }

    /// The [`Range`] of the query.
    pub(crate) fn query_range(&self) -> Option<Range<usize>> {
        Some(self.query_start()? .. self.query_after()?)
    }



    /// The query as a [`str`].
    pub fn query_str(&self) -> Option<&str> {
        Some(&self.serialization[self.query_range()?])
    }

    /// The [`MaybeQuery`].
    pub fn query(&self) -> MaybeQuery<'_> {
        unsafe {
            MaybeQuery::new_unchecked(self.query_str(), self.is_special())
        }
    }



    /// The query segments as [`str`]s.
    pub fn query_segment_strs(&self) -> Option<SplitAmpersands<'_>> {
        Some(SplitAmpersands(Some(self.query_str()?)))
    }

    /// The query segments as [`QuerySegment`]s.
    pub fn query_segments(&self) -> Option<impl DoubleEndedIterator<Item = QuerySegment<'_>>> {
        let is_special = self.is_special();

        Some(self.query_segment_strs()?.map(move |x| match is_special {
            true  => unsafe {SpecialQuerySegment   ::new_unchecked(x)}.into(),
            false => unsafe {NonSpecialQuerySegment::new_unchecked(x)}.into(),
        }))
    }

    /// The `index`th query segment as a [`str`].
    pub fn query_segment_str(&self, index: isize) -> Option<&str> {
        self.query_segment_strs()?.neg_nth(index)
    }

    /// The `index`th query segment as a [`QuerySegment`].
    pub fn query_segment(&self, index: isize) -> Option<QuerySegment<'_>> {
        self.query_segments()?.neg_nth(index)
    }

    /// If [`Self::query_segment`] is [`Some`].
    pub fn has_query_segment(&self, index: isize) -> bool {
        self.query_segment(index).is_some()
    }



    /// The query segments named `name` as [`str`]s.
    pub fn query_param_strs<'a>(&'a self, name: &str) -> Option<impl DoubleEndedIterator<Item = &'a str>> {
        Some(self.query_segment_strs()?.filter(move |x| lossy_decode_query_part(x.split_once('=').map_or(*x, |(x, _)| x)).1 == name))
    }

    /// The query segments named `name` as [`str`]s.
    pub fn query_params<'a>(&'a self, name: &str) -> Option<impl DoubleEndedIterator<Item = QuerySegment<'a>>> {
        Some(self.query_segments()?.filter(move |x| x.name() == name))
    }

    /// The `index`th query segment named `name` as a [`str`].
    pub fn query_param_str<'a>(&'a self, name: &str, index: isize) -> Option<&'a str> {
        self.query_param_strs(name)?.neg_nth(index)
    }

    /// The `index`th query segment named `name`.
    pub fn query_param<'a>(&'a self, name: &str, index: isize) -> Option<QuerySegment<'a>> {
        self.query_params(name)?.neg_nth(index)
    }

    /// If [`Self::query_param`] is [`Some`].
    pub fn has_query_param(&self, name: &str, index: isize) -> bool {
        self.query_param(name, index).is_some()
    }
}
