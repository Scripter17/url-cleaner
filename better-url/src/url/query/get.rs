//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The [`MaybeQuery`].
    pub fn query(&self) -> MaybeQuery<'_> {
        match self.is_special() {
            true  => MaybeQuery(self.query_str().map(|x| SpecialQuery   (Cow::Borrowed(x)).into())),
            false => MaybeQuery(self.query_str().map(|x| NonSpecialQuery(Cow::Borrowed(x)).into())),
        }
    }

    /// The query as a [`str`].
    pub fn query_str(&self) -> Option<&str> {
        self.url.query()
    }

    /// The [`QuerySegment`]s.
    pub fn query_segments(&self) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        let is_special = self.is_special();
        self.query_str().into_iter().flat_map(move |query| query.split('&').map(move |x| match is_special {
            true  => SpecialQuerySegment   ::new_unchecked(x).into(),
            false => NonSpecialQuerySegment::new_unchecked(x).into(),
        }))
    }

    /// The `index`th query segment named `name`.
    pub fn query_param<'a>(&'a self, name: &str, index: isize) -> Option<QuerySegment<'a>> {
        self.query_segments().filter(|s| s.name() == name).neg_nth(index)
    }
}
