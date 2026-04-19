//! Getters.

use crate::prelude::*;

impl FileSegmentedPath<'_> {
    /// The [`FilePathSegment`]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = FilePathSegment<'_>> {
        self.0[1..].split('/').map(|x| FilePathSegment(Cow::Borrowed(x)))
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
        let start = match range.start_bound().cloned() {
            Bound::Unbounded    => self.get(0)?,
            Bound::Excluded(-1) => None?,
            Bound::Excluded(x)  => self.get(x + 1)?,
            Bound::Included(x)  => self.get(x)?
        }.0.addr() - self.0.addr();

        let after = match range.end_bound().cloned() {
            Bound::Unbounded   => self.get(-1)?,
            Bound::Excluded(0) => None?,
            Bound::Excluded(x) => self.get(x - 1)?,
            Bound::Included(x) => self.get(x)?,
        }.0.end_addr() - self.0.addr();

        self.0.get(start .. after).map(|x| FilePathSegments(Cow::Borrowed(x)))
    }
}
