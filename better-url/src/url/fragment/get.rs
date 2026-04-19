//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The [`MaybeFragment`].
    pub fn fragment(&self) -> MaybeFragment<'_> {
        MaybeFragment(self.fragment_str().map(|x| Fragment(x.into())))
    }

    /// The fragment as a [`MaybeQuery`].
    pub fn fragment_query(&self) -> MaybeQuery<'_> {
        self.fragment().into()
    }

    /// The fragment.
    pub fn fragment_str(&self) -> Option<&str> {
        self.url.fragment()
    }

    /// The fragment's [`QuerySegment`]s.
    pub fn fragment_query_segments(&self) -> impl DoubleEndedIterator<Item = QuerySegment<'_>> {
        self.fragment_str().into_iter().flat_map(|x| x.split('&').map(Into::into))
    }

    /// The `index`th fragment [`QuerySegment`] named `name`.
    pub fn fragment_query_param<'a>(&'a self, name: &str, index: isize) -> Option<QuerySegment<'a>> {
        self.fragment_query_segments().filter(|s| s.name() == name).neg_nth(index)
    }
}
