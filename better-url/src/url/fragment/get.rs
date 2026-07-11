//! Getters.

use crate::prelude::*;

impl BetterUrl {
    /// The [`Range::start`] of the fragment.
    pub(crate) fn fragment_mark(&self) -> Option<usize> {
        Some(self.fragment_mark?.get() as usize + 1)
    }

    /// The [`Range::end`] of the fragment.
    pub(crate) fn fragment_after(&self) -> Option<usize> {
        match self.fragment_mark.is_some() {
            true  => Some(self.len()),
            false => None,
        }
    }

    /// The [`Range`] of the fragment.
    pub(crate) fn fragment_range(&self) -> Option<Range<usize>> {
        Some(self.fragment_mark()? .. self.fragment_after()?)
    }



    /// The fragment as a [`str`].
    pub fn fragment_str(&self) -> Option<&str> {
        Some(&self.serialization[self.fragment_range()?])
    }

    /// The [`MaybeFragment`].
    pub fn fragment(&self) -> MaybeFragment<'_> {
        unsafe {
            MaybeFragment::new_unchecked(self.fragment_str())
        }
    }

    /// The [`MaybeFragmentQuery`].
    pub fn fragment_query(&self) -> MaybeFragmentQuery<'_> {
        unsafe {
            MaybeFragmentQuery::new_unchecked(self.fragment_str())
        }
    }

    /// The fragment's [`FragmentQuerySegment`]s.
    pub fn fragment_query_segments(&self) -> impl DoubleEndedIterator<Item = FragmentQuerySegment<'_>> {
        SplitAmpersands(self.fragment_str()).map(|x| unsafe {FragmentQuerySegment::new_unchecked(x)})
    }

    /// The `index`th fragment [`FragmentQuerySegment`] named `name`.
    pub fn fragment_query_param<'a>(&'a self, name: &str, index: isize) -> Option<FragmentQuerySegment<'a>> {
        self.fragment_query_segments().filter(|s| s.name() == name).neg_nth(index)
    }
}
