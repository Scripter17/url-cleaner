//! Getters.

use crate::prelude::*;

impl NonSpecialSegmentedPath<'_> {
    /// The [`NonSpecialPathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = NonSpecialPathSegment<'_>> {
        SplitSlashes(Some(&self.0[1..])).map(|x| NonSpecialPathSegment(Cow::Borrowed(x)))
    }

    /// The `index`th [`NonSpecialPathSegment`].
    pub fn get(&self, index: isize) -> Option<NonSpecialPathSegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// Get a range of segments.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let path = NonSpecialSegmentedPath::new("/ab/cd/ef");
    ///
    /// assert_eq!(path.get_range(0..  2).unwrap(), "ab/cd");
    /// assert_eq!(path.get_range(0.. -1).unwrap(), "ab/cd");
    /// assert_eq!(path.get_range(0..= 2).unwrap(), "ab/cd/ef");
    /// assert_eq!(path.get_range(0..=-1).unwrap(), "ab/cd/ef");
    /// ```
    pub fn get_range<B: RangeBounds<isize>>(&self, range: B) -> Option<NonSpecialPathSegments<'_>> {
        path_segments_range_thing(&self.as_str()[1..], range).map(|x| unsafe {NonSpecialPathSegments::new_unchecked(x)})
    }
}
