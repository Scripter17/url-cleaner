//! Getters.

use crate::prelude::*;

impl FilePath<'_> {
    /// The segments as [`str`]s.
    pub fn iter_strs(&self) -> SplitSlashes<'_> {
        SplitSlashes(self.as_str().strip_prefix('/'))
    }

    /// The `index`th segment as a [`str`].
    pub fn get_str(&self, index: isize) -> Option<&str> {
        self.iter_strs().neg_nth(index)
    }

    /// The range of segments as a [`str`].
    pub fn range_str<B: RangeBounds<isize>>(&self, range: B) -> Option<&str> {
        self.iter_strs().range(range)
    }



    /// The [`FilePathSegmentsIter`].
    pub fn iter(&self) -> FilePathSegmentsIter<'_> {
        self.into_iter()
    }

    /// The `index`th [`FilePathSegment`].
    pub fn get(&self, index: isize) -> Option<FilePathSegment<'_>> {
        self.iter().neg_nth(index)
    }

    /// The range of [`FilePathSegments`].
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let path = FilePath::new("/ab/cd/ef");
    ///
    /// assert_eq!(path.range(0..  2).unwrap(), "ab/cd");
    /// assert_eq!(path.range(0.. -1).unwrap(), "ab/cd");
    /// assert_eq!(path.range(0..= 2).unwrap(), "ab/cd/ef");
    /// assert_eq!(path.range(0..=-1).unwrap(), "ab/cd/ef");
    /// ```
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<FilePathSegments<'_>> {
        self.iter().range(range)
    }
}
