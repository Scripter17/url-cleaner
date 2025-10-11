//! Implementing path stuff for [`BetterUrl`].

use super::*;

/// The enum of errors [`BetterUrl::set_path_segment`] can return.
#[derive(Debug, Error)]
pub enum SetPathSegmentError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when the path segment isn't found.
    #[error("The path segment wasn't found.")]
    SegmentNotFound,
    /// Returned when attempting to remove the last path segment.
    #[error("Attempted to remove the last path segment.")]
    CannotRemoveLastPathSegment,
    /// Returned when attempting to set a path segment to a value containing a `/`.
    #[error("Attempted to set a path segment to a value containing a /.")]
    PathSegmentCannotContainSlash
}

/// The enum of errors [`BetterUrl::set_path_segments_str`] can return.
#[derive(Debug, Error)]
pub enum SetPathSegmentsStrError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
}

/// The enum of errors [`BetterUrl::set_path_segments`] can return.
#[derive(Debug, Error)]
pub enum SetPathSegmentsError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to set the path to zero segments.
    #[error("Attempted to set the path to zero segments.")]
    CannotHaveZeroPathSegments,
    /// Returned when attempting to set a path segment to a value containing a `/`.
    #[error("Attempted to set a path segment to a value containing a /.")]
    PathSegmentCannotContainSlash
}

/// The enum of errors [`BetterUrl::insert_path_segment`] can return.
#[derive(Debug, Error)]
pub enum InsertPathSegmentError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when the path segment isn't found.
    #[error("The path segment wasn't found.")]
    SegmentNotFound,
    /// Returned when attempting to set a path segment to a value containing a `/`.
    #[error("Attempted to set a path segment to a value containing a /.")]
    PathSegmentCannotContainSlash
}

/// The enum of errors [`BetterUrl::remove_first_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum RemoveFirstNPathSegmentsError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to remove more path segments than are available.
    #[error("Attempted to remove more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::keep_first_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum KeepFirstNPathSegmentsError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::remove_last_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum RemoveLastNPathSegmentsError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to remove more path segments than are available.
    #[error("Attempted to remove more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::keep_last_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum KeepLastNPathSegmentsError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::set_first_n_path_segments`] can return.
#[derive(Error, Debug)]
pub enum SetFirstNPathSegmentsError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments,
    /// Returned when attempting to set a path segment to a value containing a `/`.
    #[error("Attempted to set a path segment to a value containing a /.")]
    PathSegmentCannotContainSlash
}

/// The enum of errors [`BetterUrl::set_path_segments_after_first_n`] can return.
#[derive(Error, Debug)]
pub enum SetPathSegmentsAfterFirstNError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments,
    /// Returned when attempting to set a path segment to a value containing a `/`.
    #[error("Attempted to set a path segment to a value containing a /.")]
    PathSegmentCannotContainSlash
}

/// The enum of errors [`BetterUrl::set_last_n_path_segments`] can return.
#[derive(Error, Debug)]
pub enum SetLastNPathSegmentsError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments,
    /// Returned when attempting to set a path segment to a value containing a `/`.
    #[error("Attempted to set a path segment to a value containing a /.")]
    PathSegmentCannotContainSlash
}

/// The enum of errors [`BetterUrl::set_path_segments_before_last_n`] can return.
#[derive(Error, Debug)]
pub enum SetPathSegmentsBeforeLastNError {
    /// Returned when the URL doesn't have path segments.
    #[error("The URL does not have path segments.")]
    UrlDoesNotHavePathSegments,
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments,
    /// Returned when attempting to set a path segment to a value containing a `/`.
    #[error("Attempted to set a path segment to a value containing a /.")]
    PathSegmentCannotContainSlash
}

impl BetterUrl {
    /// [`Url::set_path`].
    pub fn set_path(&mut self, path: &str) {
        self.url.set_path(path)
    }

    /// Returns [`true`] if the path has segments.
    pub fn path_has_segments(&self) -> bool {
        self.url.path().starts_with('/')
    }
    
    /// Set [`Url::path_segments`].
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If `iter` has a length of zero, returns the error [`SetPathSegmentsError::CannotHaveZeroPathSegments`].
    ///
    /// If a segment contains a `/`, returns the error [`SetPathSegmentsError::PathSegmentCannotContainSlash`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// url.set_path_segments([]).unwrap_err();
    ///
    /// url.set_path_segments(["a"]).unwrap();
    /// assert_eq!(url.path(), "/a");
    ///
    /// url.set_path_segments(["a", "b"]).unwrap();
    /// assert_eq!(url.path(), "/a/b");
    ///
    /// url.set_path_segments(["a/b/c"]).unwrap_err();
    /// assert_eq!(url.path(), "/a/b");
    /// ```
    pub fn set_path_segments<'a, I: IntoIterator<Item = &'a str>>(&mut self, iter: I) -> Result<(), SetPathSegmentsError> {
        if !self.path_has_segments() {
            Err(SetPathSegmentsError::UrlDoesNotHavePathSegments)?
        }
        let mut new = "/".to_string();
        let mut iter = iter.into_iter();
        match iter.next() {
            Some(segment) => if segment.contains('/') {
                Err(SetPathSegmentsError::PathSegmentCannotContainSlash)?
            } else {
                new.push_str(segment);
            },
            None => Err(SetPathSegmentsError::CannotHaveZeroPathSegments)?
        }
        for segment in iter {
            if segment.contains('/') {
                Err(SetPathSegmentsError::PathSegmentCannotContainSlash)?
            } else {
                new.push('/');
                new.push_str(segment);
            }
        }
        self.set_path(&new);
        Ok(())
    }
    
    /// Set [`Url::path_segments`] without checking if a segment contains a `/`.
    ///
    /// Useful for optimizations.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If `iter` has a length of zero, returns the error [`SetPathSegmentsError::CannotHaveZeroPathSegments`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// url.set_raw_path_segments([]).unwrap_err();
    ///
    /// url.set_raw_path_segments(["a"]).unwrap();
    /// assert_eq!(url.path(), "/a");
    ///
    /// url.set_raw_path_segments(["a", "b"]).unwrap();
    /// assert_eq!(url.path(), "/a/b");
    ///
    /// url.set_raw_path_segments(["a/b/c"]).unwrap();
    /// assert_eq!(url.path(), "/a/b/c");
    /// ```
    pub fn set_raw_path_segments<'a, I: IntoIterator<Item = &'a str>>(&mut self, iter: I) -> Result<(), SetPathSegmentsError> {
        if !self.path_has_segments() {
            Err(SetPathSegmentsError::UrlDoesNotHavePathSegments)?
        }
        let mut new = "/".to_string();
        let mut iter = iter.into_iter();
        match iter.next() {
            Some(segment) => new.push_str(segment),
            None => Err(SetPathSegmentsError::CannotHaveZeroPathSegments)?
        }
        for segment in iter {
            new.push('/');
            new.push_str(segment);
        }
        self.set_path(&new);
        Ok(())
    }

    /// Set [`Url::path_segments`].
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetPathSegmentsStrError::UrlDoesNotHavePathSegments`].
    pub fn set_path_segments_str(&mut self, to: &str) -> Result<(), SetPathSegmentsStrError> {
        if self.path_has_segments() {
            self.set_path(&format!("/{to}"));
        } else {
            Err(SetPathSegmentsStrError::UrlDoesNotHavePathSegments)?;
        }
        Ok(())
    }

    /// Gets the specified path segment.
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c").unwrap();
    ///
    /// assert_eq!(url.path_segment(-4), Some(None            ));
    /// assert_eq!(url.path_segment(-3), Some(Some("a".into())));
    /// assert_eq!(url.path_segment(-2), Some(Some("b".into())));
    /// assert_eq!(url.path_segment(-1), Some(Some("c".into())));
    /// assert_eq!(url.path_segment( 0), Some(Some("a".into())));
    /// assert_eq!(url.path_segment( 1), Some(Some("b".into())));
    /// assert_eq!(url.path_segment( 2), Some(Some("c".into())));
    /// assert_eq!(url.path_segment( 3), Some(None            ));
    /// ````
    pub fn path_segment(&self, index: isize) -> Option<Option<&str>> {
        match index {
            0.. => self.path_segments().map(|mut x| x.nth(index as usize)),
            ..0 => self.path_segments().map(|mut x| x.nth_back((-index - 1) as usize))
        }
    }

    /// Gets an object that can mutate the segments of [`Self`]'s path.
    /// # Examples
    /// ```
    /// use better_url::*;
    /// let mut url = BetterUrl::parse("https://example.com/a/b/c/").unwrap();
    ///
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b/c");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a/b");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/a");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// url.path_segments_mut().unwrap().pop(); assert_eq!(url.path(), "/");
    /// ```
    pub fn path_segments_mut(&mut self) -> Option<PathSegmentsMut<'_>> {
        self.url.path_segments_mut().ok()
    }

    /// [`Url::path`] with the leading `/` removed.
    ///
    /// When split on `/`, gives identical values to [`Url::path_segments`].
    pub fn path_segments_str(&self) -> Option<&str> {
        self.path().strip_prefix('/')
    }

    /// Sets the specified path segment.
    /// # Errors
    /// If a segment contains a `/`, returns the error [`SetPathSegmentsError::PathSegmentCannotContainSlash`].
    ///
    /// If the call to [`Self::set_raw_path_segment`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::*;
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
        if let Some(value) = value && value.contains('/') {
            Err(SetPathSegmentError::PathSegmentCannotContainSlash)?
        }
        self.set_raw_path_segment(index, value)
    }

    /// Sets the specified path segment without checking if the segment contains a `/`.
    ///
    /// Useful for optimizations.
    /// # Errors
    /// If the call to [`Self::path_segments_str`] returns [`None`], returns the error [`SetPathSegmentError::UrlDoesNotHavePathSegments`].
    ///
    /// If the specified path segment isn't found, returns the error [`SetPathSegmentError::SegmentNotFound`].
    pub fn set_raw_path_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetPathSegmentError> {
        match (index, value) {
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            ( 0, None)  => self.set_path(&self.path_segments_str().ok_or(SetPathSegmentError::UrlDoesNotHavePathSegments)?. split_once('/').ok_or(SetPathSegmentError::CannotRemoveLastPathSegment)?.1.to_string()),
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            (-1, None)  => self.set_path(&self.path_segments_str().ok_or(SetPathSegmentError::UrlDoesNotHavePathSegments)?.rsplit_once('/').ok_or(SetPathSegmentError::CannotRemoveLastPathSegment)?.0.to_string()),
            _ => self.set_path(&set_segment(
                self.path_segments_str().ok_or(SetPathSegmentError::UrlDoesNotHavePathSegments)?,
                "/", index, value, SetPathSegmentError::SegmentNotFound
            )?.ok_or(SetPathSegmentError::CannotRemoveLastPathSegment)?)
        }
        Ok(())
    }

    /// Inserts a path segment at the specified path segment.
    ///
    /// If the specified segment is one after the last, inserts a new segment at the end.
    /// # Errors
    /// If the call to [`Self::path_segments_str`] returns [`None`], returns the error [`InsertPathSegmentError::UrlDoesNotHavePathSegments`].
    ///
    /// If the specified path segment isn't found, returns the error [`InsertPathSegmentError::SegmentNotFound`].
    ///
    /// If a segment contains a `/`, returns the error [`SetPathSegmentsError::PathSegmentCannotContainSlash`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/").unwrap();
    ///
    /// url.insert_path_segment(0, "abc").unwrap();
    /// assert_eq!(url.path(), "/abc/");
    ///
    /// url.insert_path_segment(0, "def").unwrap();
    /// assert_eq!(url.path(), "/def/abc/");
    ///
    /// url.insert_path_segment(3, "ghi").unwrap();
    /// assert_eq!(url.path(), "/def/abc//ghi");
    ///
    /// url.insert_path_segment(6, "err").unwrap_err();
    /// assert_eq!(url.path(), "/def/abc//ghi");
    ///
    /// url.insert_path_segment(0, "a/b").unwrap_err();
    /// assert_eq!(url.path(), "/def/abc//ghi");
    /// ```
    pub fn insert_path_segment(&mut self, index: isize, value: &str) -> Result<(), InsertPathSegmentError> {
        if value.contains('/') {
            Err(InsertPathSegmentError::PathSegmentCannotContainSlash)?;
        }
        self.insert_raw_path_segment(index, value)
    }

    /// Inserts a path segment at the specified path segment without checking if a segment contains a `/`.
    ///
    /// If the specified segment is one after the last, inserts a new segment at the end.
    /// # Errors
    /// If the call to [`Self::path_segments_str`] returns [`None`], returns the error [`InsertPathSegmentError::UrlDoesNotHavePathSegments`].
    ///
    /// If the specified path segment isn't found, returns the error [`InsertPathSegmentError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/").unwrap();
    ///
    /// url.insert_raw_path_segment(0, "abc").unwrap();
    /// assert_eq!(url.path(), "/abc/");
    ///
    /// url.insert_raw_path_segment(0, "def").unwrap();
    /// assert_eq!(url.path(), "/def/abc/");
    ///
    /// url.insert_raw_path_segment(3, "ghi").unwrap();
    /// assert_eq!(url.path(), "/def/abc//ghi");
    ///
    /// url.insert_raw_path_segment(6, "err").unwrap_err();
    /// assert_eq!(url.path(), "/def/abc//ghi");
    ///
    /// url.insert_raw_path_segment(0, "a/b").unwrap();
    /// assert_eq!(url.path(), "/a/b/def/abc//ghi");
    /// ```
    pub fn insert_raw_path_segment(&mut self, index: isize, value: &str) -> Result<(), InsertPathSegmentError> {
        self.set_path(&insert_segment(
            self.path_segments_str().ok_or(InsertPathSegmentError::UrlDoesNotHavePathSegments)?,
            "/", index, value, InsertPathSegmentError::SegmentNotFound
        )?);

        Ok(())
    }

    /// Get the first `n` path segments.
    pub fn first_n_path_segments(&self, n: usize) -> Option<Option<&str>> {
        self.path_segments_str().map(|x| char_keep_first_n_segments(x, '/', n))
    }

    /// Gets the path segments except for the first `n`.
    pub fn path_segments_after_first_n(&self, n: usize) -> Option<Option<&str>> {
        self.path_segments_str().map(|x| char_remove_first_n_segments(x, '/', n))
    }

    /// Get the last `n` path segments.
    pub fn last_n_path_segments(&self, n: usize) -> Option<Option<&str>> {
        self.path_segments_str().map(|x| char_keep_last_n_segments(x, '/', n))
    }

    /// Gets the path segments except for the last `n`.
    pub fn path_segments_before_last_n(&self, n: usize) -> Option<Option<&str>> {
        self.path_segments_str().map(|x| char_remove_last_n_segments(x, '/', n))
    }

    /// Sets the first `n` path segments.
    ///
    /// Does not require `to` to have `n` segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetFirstNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetFirstNPathSegmentsError::NotEnoughPathSegments`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/0/1/2/3/4/5").unwrap();
    ///
    /// url.set_first_n_path_segments(0, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/3/4/5");
    /// url.set_first_n_path_segments(0, None).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/3/4/5");
    ///
    /// url.set_first_n_path_segments(1, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a/1/2/3/4/5");
    /// url.set_first_n_path_segments(1, None).unwrap();
    /// assert_eq!(url.path(), "/1/2/3/4/5");
    ///
    /// url.set_first_n_path_segments(2, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a/3/4/5");
    /// url.set_first_n_path_segments(2, None).unwrap();
    /// assert_eq!(url.path(), "/4/5");
    ///
    /// url.set_first_n_path_segments(3, Some("a")).unwrap_err();
    /// assert_eq!(url.path(), "/4/5");
    /// url.set_first_n_path_segments(3, None).unwrap_err();
    /// assert_eq!(url.path(), "/4/5");
    /// ```
    pub fn set_first_n_path_segments(&mut self, n: usize, to: Option<&str>) -> Result<(), SetFirstNPathSegmentsError> {
        if n > 0 {
            match to {
                Some(to) => self.set_path(&format!("/{to}/{}", char_remove_first_n_segments(self.path_segments_str().ok_or(SetFirstNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetFirstNPathSegmentsError::NotEnoughPathSegments)?)),
                None     => self.set_path(&format!("/{}"     , char_remove_first_n_segments(self.path_segments_str().ok_or(SetFirstNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetFirstNPathSegmentsError::NotEnoughPathSegments)?)),
            }
        }
        Ok(())
    }

    /// Sets the path segments except for the first `n`.
    ///
    /// Does not require `to` to have `n` segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetFirstNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetFirstNPathSegmentsError::NotEnoughPathSegments`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/0/1/2/3/4/5").unwrap();
    ///
    /// url.set_path_segments_after_first_n(3, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/a");
    /// url.set_path_segments_after_first_n(3, None).unwrap();
    /// assert_eq!(url.path(), "/0/1/2");
    ///
    /// url.set_path_segments_after_first_n(2, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/0/1/a");
    /// url.set_path_segments_after_first_n(2, None).unwrap();
    /// assert_eq!(url.path(), "/0/1");
    ///
    /// url.set_path_segments_after_first_n(1, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/0/a");
    /// url.set_path_segments_after_first_n(1, None).unwrap();
    /// assert_eq!(url.path(), "/0");
    ///
    /// url.set_path_segments_after_first_n(0, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a");
    /// url.set_path_segments_after_first_n(0, None).unwrap_err();
    /// assert_eq!(url.path(), "/a");
    /// ```
    pub fn set_path_segments_after_first_n(&mut self, n: usize, to: Option<&str>) -> Result<(), SetPathSegmentsAfterFirstNError> {
        match (n, to) {
            (0, Some(to)) => self.set_path(&format!("/{to}")),
            (_, Some(to)) => self.set_path(&format!("/{}/{to}", char_keep_first_n_segments(self.path_segments_str().ok_or(SetPathSegmentsAfterFirstNError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetPathSegmentsAfterFirstNError::NotEnoughPathSegments)?)),
            (_, None    ) => self.set_path(&format!("/{}"     , char_keep_first_n_segments(self.path_segments_str().ok_or(SetPathSegmentsAfterFirstNError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetPathSegmentsAfterFirstNError::NotEnoughPathSegments)?)),
        }
        Ok(())
    }

    /// Sets the last `n` path segments.
    ///
    /// Does not require `to` to have `n` segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetLastNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetLastNPathSegmentsError::NotEnoughPathSegments`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/0/1/2/3/4/5").unwrap();
    ///
    /// url.set_last_n_path_segments(0, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/3/4/5");
    /// url.set_last_n_path_segments(0, None).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/3/4/5");
    ///
    /// url.set_last_n_path_segments(1, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/3/4/a");
    /// url.set_last_n_path_segments(1, None).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/3/4");
    ///
    /// url.set_last_n_path_segments(2, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/0/1/2/a");
    /// url.set_last_n_path_segments(2, None).unwrap();
    /// assert_eq!(url.path(), "/0/1");
    ///
    /// url.set_last_n_path_segments(3, Some("a")).unwrap_err();
    /// assert_eq!(url.path(), "/0/1");
    /// url.set_last_n_path_segments(3, None).unwrap_err();
    /// assert_eq!(url.path(), "/0/1");
    /// ```
    pub fn set_last_n_path_segments(&mut self, n: usize, to: Option<&str>) -> Result<(), SetLastNPathSegmentsError> {
        if n > 0 {
            match to {
                Some(to) => self.set_path(&format!("/{}/{to}", char_remove_last_n_segments(self.path_segments_str().ok_or(SetLastNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetLastNPathSegmentsError::NotEnoughPathSegments)?)),
                None     => self.set_path(&format!("/{}"     , char_remove_last_n_segments(self.path_segments_str().ok_or(SetLastNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetLastNPathSegmentsError::NotEnoughPathSegments)?)),
            }
        }
        Ok(())
    }

    /// Sets the path segments except for the last `n`.
    ///
    /// Does not require `to` to have `n` segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetLastNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetLastNPathSegmentsError::NotEnoughPathSegments`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://example.com/0/1/2/3/4/5").unwrap();
    ///
    /// url.set_path_segments_before_last_n(3, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a/3/4/5");
    /// url.set_path_segments_before_last_n(3, None).unwrap();
    /// assert_eq!(url.path(), "/3/4/5");
    ///
    /// url.set_path_segments_before_last_n(2, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a/4/5");
    /// url.set_path_segments_before_last_n(2, None).unwrap();
    /// assert_eq!(url.path(), "/4/5");
    ///
    /// url.set_path_segments_before_last_n(1, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a/5");
    /// url.set_path_segments_before_last_n(1, None).unwrap();
    /// assert_eq!(url.path(), "/5");
    ///
    /// url.set_path_segments_before_last_n(0, Some("a")).unwrap();
    /// assert_eq!(url.path(), "/a");
    /// url.set_path_segments_before_last_n(0, None).unwrap_err();
    /// assert_eq!(url.path(), "/a");
    /// ```
    pub fn set_path_segments_before_last_n(&mut self, n: usize, to: Option<&str>) -> Result<(), SetPathSegmentsBeforeLastNError> {
        match (n, to) {
            (0, Some(to)) => self.set_path(&format!("/{to}")),
            (_, Some(to)) => self.set_path(&format!("/{to}/{}", char_keep_last_n_segments(self.path_segments_str().ok_or(SetPathSegmentsBeforeLastNError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetPathSegmentsBeforeLastNError::NotEnoughPathSegments)?)),
            (_, None    ) => self.set_path(&format!("/{}"     , char_keep_last_n_segments(self.path_segments_str().ok_or(SetPathSegmentsBeforeLastNError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(SetPathSegmentsBeforeLastNError::NotEnoughPathSegments)?)),
        }
        Ok(())
    }

    /// Remove the first `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to the number of path segments before this is applied minus `n`.
    ///
    /// Because a path can't have zero segments, trying to remove all segments counts as not having enough segments. If this is a serious ergonomics issue for you, I'll prioritize making a workaround.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`RemoveFirstNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`RemoveFirstNPathSegmentsError::NotEnoughPathSegments`].
    pub fn remove_first_n_path_segments(&mut self, n: usize) -> Result<(), RemoveFirstNPathSegmentsError> {
        self.set_path(&format!("/{}", char_remove_first_n_segments(self.path_segments_str().ok_or(RemoveFirstNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(RemoveFirstNPathSegmentsError::NotEnoughPathSegments)?));
        Ok(())
    }

    /// Keep the first `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to `n`.
    ///
    /// Because a path can't have zero segments, trying to keep zero segments always errors. This is easy to just not do.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`KeepFirstNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`KeepFirstNPathSegmentsError::NotEnoughPathSegments`].
    pub fn keep_first_n_path_segments(&mut self, n: usize) -> Result<(), KeepFirstNPathSegmentsError> {
        self.set_path(&format!("/{}", char_keep_first_n_segments(self.path_segments_str().ok_or(KeepFirstNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(KeepFirstNPathSegmentsError::NotEnoughPathSegments)?));
        Ok(())
    }

    /// Remove the last `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to the number of path segments before this is applied minus `n`.
    ///
    /// Because a path can't have zero segments, trying to remove all segments counts as not having enough segments. If this is a serious ergonomics issue for you, I'll prioritize making a workaround.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`RemoveLastNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`RemoveLastNPathSegmentsError::NotEnoughPathSegments`].
    pub fn remove_last_n_path_segments(&mut self, n: usize) -> Result<(), RemoveLastNPathSegmentsError> {
        self.set_path(&format!("/{}", char_remove_last_n_segments(self.path_segments_str().ok_or(RemoveLastNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(RemoveLastNPathSegmentsError::NotEnoughPathSegments)?));
        Ok(())
    }

    /// Keep the last `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to `n`.
    ///
    /// Because a path can't have zero segments, trying to keep zero segments always errors. This is easy to just not do.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`KeepLastNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`KeepLastNPathSegmentsError::NotEnoughPathSegments`].
    pub fn keep_last_n_path_segments(&mut self, n: usize) -> Result<(), KeepLastNPathSegmentsError> {
        self.set_path(&format!("/{}", char_keep_last_n_segments(self.path_segments_str().ok_or(KeepLastNPathSegmentsError::UrlDoesNotHavePathSegments)?, '/', n).ok_or(KeepLastNPathSegmentsError::NotEnoughPathSegments)?));
        Ok(())
    }
}
