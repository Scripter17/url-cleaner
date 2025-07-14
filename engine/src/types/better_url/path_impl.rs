//! Implementing path stuff for [`BetterUrl`].

use super::*;

/// The enum of errors [`BetterUrl::set_path_segment`] can return.
#[derive(Debug, Error)]
pub enum SetPathSegmentError {
    /// Returned when the URL doesn't have path segments.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when the path segment isn't found.
    #[error("The path segment wasn't found.")]
    SegmentNotFound
}

/// The enum of errors [`BetterUrl::insert_path_segment_at`] and [`BetterUrl::insert_path_segment_after`] can return.
#[derive(Debug, Error)]
pub enum InsertPathSegmentError {
    /// Returned when the URL doesn't have path segments.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when the path segment isn't found.
    #[error("The path segment wasn't found.")]
    SegmentNotFound
}

impl BetterUrl {
    /// [`Url::set_path`].
    pub fn set_path(&mut self, path: &str) {
        debug!(BetterUrl::set_path, self, path);
        self.url.set_path(path)
    }

    /// Returns [`true`] if the path has segments.
    pub fn path_has_segments(&self) -> bool {
        self.url.path().starts_with('/')
    }
    /// Gets an object that can iterate over the segments of [`Self`]'s path.
    /// # Errors
    #[doc = edoc!(callnone(Url::path_segments, UrlDoesNotHavePathSegments))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().path_segments().unwrap().collect::<Vec<_>>(), [""]);
    /// assert_eq!(BetterUrl::parse("https://example.com/a/b/c" ).unwrap().path_segments().unwrap().collect::<Vec<_>>(), ["a", "b", "c"]);
    /// assert_eq!(BetterUrl::parse("https://example.com/a/b/c/").unwrap().path_segments().unwrap().collect::<Vec<_>>(), ["a", "b", "c", ""]);
    /// ```
    pub fn path_segments(&self) -> Result<Split<'_, char>, UrlDoesNotHavePathSegments> {
        self.url.path_segments().ok_or(UrlDoesNotHavePathSegments)
    }

    /// Gets the specified path segment.
    /// # Errors
    #[doc = edoc!(callerr(Self::path_segments))]
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(url.path_segment(-4).unwrap(), None            );
    /// assert_eq!(url.path_segment(-3).unwrap(), Some("a".into()));
    /// assert_eq!(url.path_segment(-2).unwrap(), Some("b".into()));
    /// assert_eq!(url.path_segment(-1).unwrap(), Some("c".into()));
    /// assert_eq!(url.path_segment( 0).unwrap(), Some("a".into()));
    /// assert_eq!(url.path_segment( 1).unwrap(), Some("b".into()));
    /// assert_eq!(url.path_segment( 2).unwrap(), Some("c".into()));
    /// assert_eq!(url.path_segment( 3).unwrap(), None            );
    /// ````
    pub fn path_segment(&self, index: isize) -> Result<Option<&str>, UrlDoesNotHavePathSegments> {
        Ok(match index {
            0.. => self.path_segments()?.nth(index as usize),
            #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
            ..0 => self.path_segments()?.nth_back((-index - 1) as usize)
        })
    }

    /// Gets an object that can mutate the segments of [`Self`]'s path.
    /// # Errors
    /// If the call to [`Url::path_segments_mut`] returns an error, returns the error [`UrlDoesNotHavePathSegments`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c/").unwrap();
    ///
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b/c");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// ```
    pub fn path_segments_mut(&mut self) -> Result<PathSegmentsMut<'_>, UrlDoesNotHavePathSegments> {
        debug!(BetterUrl::path_segments_mut, self);
        self.url.path_segments_mut().map_err(|()| UrlDoesNotHavePathSegments)
    }

    /// [`Url::path`] with the leading `/` removed.
    ///
    /// When split on `/`, gives identical values to [`Self::path_segments`].
    pub fn path_segments_str(&self) -> Option<&str> {
        self.path().strip_prefix('/')
    }

    /// Sets the specified path segment.
    /// # Errors
    #[doc = edoc!(callnone(Self::path_segments_str, UrlDoesNotHavePathSegments))]
    ///
    /// If the specified path segment isn't found, returns the error [`SetPathSegmentError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/aa/bb/cc").unwrap();
    ///
    /// url.set_path_segment(-4, Some("-4")).unwrap_err(); assert_eq!(url.path(), "/aa/bb/cc");
    /// url.set_path_segment(-3, Some("-3")).unwrap    (); assert_eq!(url.path(), "/-3/bb/cc");
    /// url.set_path_segment(-2, Some("-2")).unwrap    (); assert_eq!(url.path(), "/-3/-2/cc");
    /// url.set_path_segment(-1, Some("-1")).unwrap    (); assert_eq!(url.path(), "/-3/-2/-1");
    /// url.set_path_segment( 0, Some("00")).unwrap    (); assert_eq!(url.path(), "/00/-2/-1");
    /// url.set_path_segment( 1, Some("+1")).unwrap    (); assert_eq!(url.path(), "/00/+1/-1");
    /// url.set_path_segment( 2, Some("+2")).unwrap    (); assert_eq!(url.path(), "/00/+1/+2");
    /// url.set_path_segment( 3, Some("+3")).unwrap_err(); assert_eq!(url.path(), "/00/+1/+2");
    ///
    /// url.set_path_segment( 0, None).unwrap(); assert_eq!(url.path(), "/+1/+2");
    /// url.set_path_segment(-1, None).unwrap(); assert_eq!(url.path(), "/+1");
    /// ````
    pub fn set_path_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetPathSegmentError> {
        self.set_path(&set_segment(
            self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?,
            index, value, SetPathSegmentError::SegmentNotFound, '/'
        )?.join("/"));
        Ok(())
    }

    /// Inserts a path segment at the specified path segment.
    ///
    /// If the specified segment is one after the last, inserts a new segment at the end.
    /// # Errors
    #[doc = edoc!(callnone(Self::path_segments_str, UrlDoesNotHavePathSegments))]
    ///
    /// If the specified path segment isn't found, returns the error [`InsertPathSegmentError::SegmentNotFound`].
    pub fn insert_path_segment_at(&mut self, index: isize, value: &str) -> Result<(), InsertPathSegmentError> {
        self.set_path(&insert_segment_at(
            self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?,
            index, value, InsertPathSegmentError::SegmentNotFound, '/', "/"
        )?);
        Ok(())
    }

    /// Inserts a path segment after the specified path segment.
    ///
    /// If the specified segment is one after the last, inserts a new segment at the end.
    /// # Errors
    #[doc = edoc!(callnone(Self::path_segments_str, UrlDoesNotHavePathSegments))]
    ///
    /// If the specified path segment isn't found, returns the error [`InsertPathSegmentError::SegmentNotFound`].
    pub fn insert_path_segment_after(&mut self, index: isize, value: &str) -> Result<(), InsertPathSegmentError> {
        self.set_path(&insert_segment_after(
            self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?,
            index, value, InsertPathSegmentError::SegmentNotFound, '/', "/"
        )?);
        Ok(())
    }
}
