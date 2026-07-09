//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The query as a [`str`].
    pub fn query_str(&self) -> Option<&str> {
        self.url.query()
    }

    /// The [`MaybeQuery`].
    pub fn query(&self) -> MaybeQuery<'_> {
        match self.is_special() {
            true  => MaybeQuery::Special   (self.query_str().map(SpecialQuery   ::new_unchecked).into()),
            false => MaybeQuery::NonSpecial(self.query_str().map(NonSpecialQuery::new_unchecked).into()),
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
            true  => SpecialQuerySegment   ::new_unchecked(x).into(),
            false => NonSpecialQuerySegment::new_unchecked(x).into(),
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
