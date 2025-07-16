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
    SegmentNotFound,
    /// Returned when attempting to remove the last path segment.
    #[error("Attempted to remove the last path segment.")]
    CannotRemoveLastPathSegment
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

/// The enum of errors [`BetterUrl::remove_first_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum RemoveFirstNPathSegmentsError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to remove more path segments than are available.
    #[error("Attempted to remove more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::keep_first_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum KeepFirstNPathSegmentsError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::remove_last_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum RemoveLastNPathSegmentsError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to remove more path segments than are available.
    #[error("Attempted to remove more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::keep_last_n_path_segments`] can return.
#[derive(Debug, Error)]
pub enum KeepLastNPathSegmentsError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::set_first_n_path_segments`] can return.
#[derive(Error, Debug)]
pub enum SetFirstNPathSegmentsError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::set_path_segments_after_first_n`] can return.
#[derive(Error, Debug)]
pub enum SetPathSegmentsAfterFirstNError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::set_last_n_path_segments`] can return.
#[derive(Error, Debug)]
pub enum SetLastNPathSegmentsError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
}

/// The enum of errors [`BetterUrl::set_path_segments_before_last_n`] can return.
#[derive(Error, Debug)]
pub enum SetPathSegmentsBeforeLastNError {
    /// Returned when a [`UrlDoesNotHavePathSegments`] is encountered.
    #[error(transparent)]
    UrlDoesNotHavePathSegments(#[from] UrlDoesNotHavePathSegments),
    /// Returned when attempting to keep more path segments than are available.
    #[error("Attempted to keep more path segments than were available.")]
    NotEnoughPathSegments
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
    /// Set [`Self::path_segments`].
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    pub fn set_path_segments_str(&mut self, to: &str) -> Result<(), UrlDoesNotHavePathSegments> {
        if self.path_has_segments() {
            self.set_path(to);
        } else {
            Err(UrlDoesNotHavePathSegments)?;
        }
        Ok(())
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
        match (index, value) {
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            ( 0, None)  => self.set_path(&self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?. split_once('/').ok_or(SetPathSegmentError::CannotRemoveLastPathSegment)?.1.to_string()),
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            (-1, None)  => self.set_path(&self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?.rsplit_once('/').ok_or(SetPathSegmentError::CannotRemoveLastPathSegment)?.0.to_string()),
            _ => self.set_path(&set_segment_str(
                self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?,
                index, value, SetPathSegmentError::SegmentNotFound, '/', "/"
            )?.ok_or(SetPathSegmentError::CannotRemoveLastPathSegment)?)
        }
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

    /// Get the first `n` path segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    pub fn first_n_path_segments(&self, n: usize) -> Result<Option<&str>, UrlDoesNotHavePathSegments> {
        Ok(keep_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n))
    }

    /// Gets the path segments except for the first `n`.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    pub fn path_segments_after_first_n(&self, n: usize) -> Result<Option<&str>, UrlDoesNotHavePathSegments> {
        Ok(remove_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n))
    }

    /// Get the last `n` path segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    pub fn last_n_path_segments(&self, n: usize) -> Result<Option<&str>, UrlDoesNotHavePathSegments> {
        Ok(keep_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n))
    }

    /// Gets the path segments except for the last `n`.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    pub fn path_segments_before_last_n(&self, n: usize) -> Result<Option<&str>, UrlDoesNotHavePathSegments> {
        Ok(remove_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n))
    }

    /// Sets the first `n` path segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetFirstNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetFirstNPathSegmentsError::NotEnoughPathSegments`].
    pub fn set_first_n_path_segments(&mut self, n: usize, to: Option<&str>) -> Result<(), SetFirstNPathSegmentsError> {
        match to {
            Some(to) => self.set_path_segments_str(&format!("{to}/{}", remove_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetFirstNPathSegmentsError::NotEnoughPathSegments)?))?,
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            None     => self.set_path_segments_str(&                   remove_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetFirstNPathSegmentsError::NotEnoughPathSegments)?.to_string())?,
        }
        Ok(())
    }

    /// Sets the path segments except for the first `n`.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetFirstNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetFirstNPathSegmentsError::NotEnoughPathSegments`].
    pub fn set_path_segments_after_first_n(&mut self, n: usize, to: Option<&str>) -> Result<(), SetPathSegmentsAfterFirstNError> {
        match to {
            Some(to) => self.set_path_segments_str(&format!("{}/{to}", keep_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetPathSegmentsAfterFirstNError::NotEnoughPathSegments)?))?,
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            None     => self.set_path_segments_str(&                   keep_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetPathSegmentsAfterFirstNError::NotEnoughPathSegments)?.to_string())?,
        }
        Ok(())
    }

    /// Sets the last `n` path segments.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetLastNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetLastNPathSegmentsError::NotEnoughPathSegments`].
    pub fn set_last_n_path_segments(&mut self, n: usize, to: Option<&str>) -> Result<(), SetLastNPathSegmentsError> {
        match to {
            Some(to) => self.set_path_segments_str(&format!("{to}/{}", keep_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetLastNPathSegmentsError::NotEnoughPathSegments)?))?,
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            None     => self.set_path_segments_str(&                   keep_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetLastNPathSegmentsError::NotEnoughPathSegments)?.to_string())?,
        }
        Ok(())
    }

    /// Sets the path segments except for the last `n`.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`SetLastNPathSegmentsError::UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough path segments, returns the error [`SetLastNPathSegmentsError::NotEnoughPathSegments`].
    pub fn set_path_segments_before_last_n(&mut self, n: usize, to: Option<&str>) -> Result<(), SetPathSegmentsBeforeLastNError> {
        match to {
            Some(to) => self.set_path_segments_str(&format!("{}/{to}", remove_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetPathSegmentsBeforeLastNError::NotEnoughPathSegments)?))?,
            #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
            None     => self.set_path_segments_str(&                   remove_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(SetPathSegmentsBeforeLastNError::NotEnoughPathSegments)?.to_string())?,
        }
        Ok(())
    }

    /// Remove the first `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to the number of path segments before this is applied minus `n`.
    ///
    /// Because a path can't have zero segments, this means trying to remove all segments counts as not having enough segments. If this is a serious ergonomics issue for you, I'll prioritize making a workaround.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`ActionError::NotEnoughPathSegments`].
    pub fn remove_first_n_path_segments(&mut self, n: usize) -> Result<(), RemoveFirstNPathSegmentsError> {
        #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
        self.set_path_segments_str(&remove_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(RemoveFirstNPathSegmentsError::NotEnoughPathSegments)?.to_string())?;
        Ok(())
    }

    /// Keep the first `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to `n`.
    ///
    /// Because a path can't have zero segments, this means trying to keep zero segments always errors. This is easy to just not do.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`ActionError::NotEnoughPathSegments`].
    pub fn keep_first_n_path_segments(&mut self, n: usize) -> Result<(), KeepFirstNPathSegmentsError> {
        #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
        self.set_path_segments_str(&keep_first_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(KeepFirstNPathSegmentsError::NotEnoughPathSegments)?.to_string())?;
        Ok(())
    }

    /// Remove the last `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to the number of path segments before this is applied minus `n`.
    ///
    /// Because a path can't have zero segments, this means trying to remove all segments counts as not having enough segments. If this is a serious ergonomics issue for you, I'll prioritize making a workaround.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`ActionError::NotEnoughPathSegments`].
    pub fn remove_last_n_path_segments(&mut self, n: usize) -> Result<(), RemoveLastNPathSegmentsError> {
        #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
        self.set_path_segments_str(&remove_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(RemoveLastNPathSegmentsError::NotEnoughPathSegments)?.to_string())?;
        Ok(())
    }

    /// Keep the last `n` path segments.
    ///
    /// The number of path segments after this succeeds is equal to `n`.
    ///
    /// Because a path can't have zero segments, this means trying to keep zero segments always errors. This is easy to just not do.
    /// # Errors
    /// If the URL doesn't have path segments, returns the error [`UrlDoesNotHavePathSegments`].
    ///
    /// If there aren't enough segments, returns the error [`ActionError::NotEnoughPathSegments`].
    pub fn keep_last_n_path_segments(&mut self, n: usize) -> Result<(), KeepLastNPathSegmentsError> {
        #[expect(clippy::unnecessary_to_owned, reason = "False positive.")]
        self.set_path_segments_str(&keep_last_n_segments(self.path_segments_str().ok_or(UrlDoesNotHavePathSegments)?, "/", n).ok_or(KeepLastNPathSegmentsError::NotEnoughPathSegments)?.to_string())?;
        Ok(())
    }
}
