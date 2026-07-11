//! Getters.

use crate::prelude::*;

impl FileSegmentedPath<'_> {
    /// The [`FilePathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = FilePathSegment<'_>> {
        SplitSlashes(Some(&self.0[1..])).map(|x| FilePathSegment(Cow::Borrowed(x)))
    }

    /// The `index`th [`FilePathSegment`].
    pub fn get(&self, index: isize) -> Option<FilePathSegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// Get a range of segments.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let path = FileSegmentedPath::new("/ab/cd/ef");
    ///
    /// assert_eq!(path.get_range(0..  2).unwrap(), "ab/cd");
    /// assert_eq!(path.get_range(0.. -1).unwrap(), "ab/cd");
    /// assert_eq!(path.get_range(0..= 2).unwrap(), "ab/cd/ef");
    /// assert_eq!(path.get_range(0..=-1).unwrap(), "ab/cd/ef");
    /// ```
    pub fn get_range<B: RangeBounds<isize>>(&self, range: B) -> Option<FilePathSegments<'_>> {
        path_segments_range_thing(&self.as_str()[1..], range).map(|x| unsafe {FilePathSegments::new_unchecked(x)})
    }
}
