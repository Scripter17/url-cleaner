//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Make a new [`Path`] for this URL.
    pub fn new_path<'a, T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>> + Into<OpaquePath<'a>>>(&self, path: T) -> Path<'a> {
        match self.cannot_be_a_base() {
            true  => Path::new_opaque(path),
            false => match self.scheme_type() {
                SchemeType::File           => Path::new_file            (path),
                SchemeType::SpecialNotFile => Path::new_special_not_file(path),
                SchemeType::NonSpecial     => Path::new_non_special     (path),
            }
        }
    }

    /// Set the path.
    /// # Errors
    /// If setting a path that's too long, returns the error [`TooLong`].
    pub fn set_path<'a, T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>> + Into<OpaquePath<'a>>>(&mut self, path: T) -> Result<bool, SetPathError> {
        let old = self.path_str();

        let new = self.new_path(path);

        if old == new {
            return Ok(false);
        }

        let new_len = self.len() - old.len() + new.len();

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        self.url.set_path(new.as_str());

        debug_assert_eq!(self.len (), new_len);
        debug_assert_eq!(self.path(), new    );

        Ok(true)
    }

    /// [`SegmentedPath::set`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::set`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn set_path_segment<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.set(index, value)? {
            self.set_path(path.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`SegmentedPath::set_range`];
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::set_range`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn set_path_range<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>, I: IntoIterator<Item = T>, B: RangeBounds<isize>>(&mut self, range: B, iter: I) -> Result<bool, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.set_range(range, iter)? {
            self.set_path(path.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`SegmentedPath::insert`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::insert`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn insert_path_segment<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>>(&mut self, index: isize, value: T) -> Result<bool, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.insert(index, value)? {
            self.set_path(path.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`SegmentedPath::insert_segments`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::insert_segments`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn insert_path_segments<'a, T: Into<SpecialNotFilePathSegment<'a>> + Into<FilePathSegment<'a>> + Into<NonSpecialPathSegment<'a>>, I: IntoIterator<Item = T>>(&mut self, index: isize, iter: I) -> Result<bool, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.insert_segments(index, iter)? {
            self.set_path(path.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
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
    pub fn pop_path_if_empty(&mut self) -> Result<bool, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.pop_if_empty()? {
            self.set_path(path.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// [`SegmentedPath::remove`].
    /// # Errors
    /// If the call to [`Self::segmented_path`] returns [`None`], returns the error [`PathIsOpaque`].
    ///
    /// If the call to [`SegmentedPath::remove`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn remove_path_segment(&mut self, index: isize) -> Result<(), SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        path.remove(index)?;

        self.set_path(path.into_owned())?;

        Ok(())
    }
}
