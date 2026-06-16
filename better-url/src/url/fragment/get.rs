//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The fragment.
    pub fn fragment_str(&self) -> Option<&str> {
        self.url.fragment()
    }

    /// The [`MaybeFragment`].
    pub fn fragment(&self) -> MaybeFragment<'_> {
        MaybeFragment(self.fragment_str().map(Fragment::new_unchecked))
    }

    /// The [`MaybeFragmentQuery`].
    pub fn fragment_query(&self) -> MaybeFragmentQuery<'_> {
        MaybeFragmentQuery(self.fragment_str().map(FragmentQuery::new_unchecked))
    }

    /// The fragment's [`FragmentQuerySegment`]s.
    pub fn fragment_query_segments(&self) -> impl DoubleEndedIterator<Item = FragmentQuerySegment<'_>> {
        self.fragment_str().into_iter().flat_map(|x| x.split('&').map(FragmentQuerySegment::new_unchecked))
    }

    /// The `index`th fragment [`FragmentQuerySegment`] named `name`.
    pub fn fragment_query_param<'a>(&'a self, name: &str, index: isize) -> Option<FragmentQuerySegment<'a>> {
        self.fragment_query_segments().filter(|s| s.name() == name).neg_nth(index)
    }
}
