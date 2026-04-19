//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Set the path.
    /// # Errors
    /// If setting a path that's too long, returns the error [`TooLong`].
    pub fn set_path<'a, T: Into<SpecialNotFilePath<'a>> + Into<FilePath<'a>> + Into<NonSpecialPath<'a>> + Into<OpaquePath<'a>>>(&mut self, path: T) -> Result<(), SetPathError> {
        let new = match (self.scheme_type(), self.cannot_be_a_base()) {
            (_                         , true) => Path::new_opaque          (path),
            (SchemeType::SpecialNotFile, _   ) => Path::new_special_not_file(path),
            (SchemeType::File          , _   ) => Path::new_file            (path),
            (SchemeType::NonSpecial    , _   ) => Path::new_non_special     (path),
        };

        let old = self.path_str();

        if old == new {
            return Ok(());
        }

        let new_len = self.len() - old.len() + new.len();

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        self.url.set_path(new.as_str());

        debug_assert_eq!(self.len(), new_len);
        debug_assert_eq!(self.path(), new);

        Ok(())
    }

    /// [`SegmentedPath::set`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::set`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn set_path_segment<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.set(index, value)? {
            self.set_path(path.into_owned())?;
        }

        Ok(())
    }

    /// [`SegmentedPath::set_segments`];
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::set_segments`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn set_path_segments<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>, I: IntoIterator<Item = T>, B: RangeBounds<isize>>(&mut self, range: B, iter: I) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.set_segments(range, iter)? {
            self.set_path(path.into_owned())?;
        }

        Ok(())
    }

    /// [`SegmentedPath::insert`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::insert`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn insert_path_segment<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: T) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.insert(index, value)? {
            self.set_path(path.into_owned())?;
        }

        Ok(())
    }

    /// [`SegmentedPath::insert_segments`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::insert_segments`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn insert_path_segments<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>, I: IntoIterator<Item = T>>(&mut self, index: isize, iter: I) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.insert_segments(index, iter)? {
            self.set_path(path.into_owned())?;
        }

        Ok(())
    }

    /// [`SegmentedPath::pop`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::pop`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn pop_path(&mut self) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        path.pop()?;

        self.set_path(path.into_owned())?;

        Ok(())
    }

    /// [`SegmentedPath::pop_if_empty`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::pop_if_empty`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn pop_path_if_empty(&mut self) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.pop_if_empty()? {
            self.set_path(path.into_owned())?;
        }

        Ok(())
    }



    /// Modify the path.
    /// # Errors
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn modify_path<F: FnOnce(&mut Path<'_>)>(&mut self, f: F) -> Result<(), SetPathError> {
        let mut path = self.path();
        let old = path.clone();

        f(&mut path);

        if old != path {
            self.set_path(path.into_owned())?;
        }

        Ok(())
    }

    /// Modify the path.
    /// # Errors
    /// If the call to `f` returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn try_modify_path<F: FnOnce(&mut Path<'_>) -> Result<(), E>, E>(&mut self, f: F) -> Result<Result<(), E>, SetPathError> {
        let mut path = self.path();
        let old = path.clone();

        if let Err(e) = f(&mut path) {
            return Ok(Err(e))
        }

        if old != path {
            self.set_path(path.into_owned())?;
        }

        Ok(Ok(()))
    }

    /// Modify the [`SegmentedPath`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn modify_segmented_path<F: FnOnce(&mut SegmentedPath<'_>)>(&mut self, f: F) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;
        let old = path.clone();

        f(&mut path);

        if old != path {
            self.set_path(path.into_owned())?;
        }

        Ok(())
    }

    /// Modify the [`SegmentedPath`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to `f` returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn try_modify_segmented_path<F: FnOnce(&mut SegmentedPath<'_>) -> Result<(), E>, E>(&mut self, f: F) -> Result<Result<(), E>, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;
        let old = path.clone();

        if let Err(e) = f(&mut path) {
            return Ok(Err(e));
        }

        if old != path {
            self.set_path(path.into_owned())?;
        }

        Ok(Ok(()))
    }
}
