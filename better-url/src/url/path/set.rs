//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Set the path.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    pub fn set_path<'a, T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>> + Into<NonSpecialSegmentedPath<'a>>>(&mut self, path: T) -> Result<(), SetPathError> {
        if self.cannot_be_a_base() {
            return Ok(());
        }

        let new = match self.details.scheme.r#type() {
            SchemeType::File           => Path::new_file            (path),
            SchemeType::SpecialNotFile => Path::new_special_not_file(path),
            SchemeType::NonSpecial     => match self.has_host() {
                true  => Path::new_non_special          (path),
                false => Path::new_non_special_segmented(path),
            }
        };

        let a = self.has_host();
        let b = &self.serialization[self.scheme_mark as usize .. self.path_start as usize] == ":/.";
        let c = new.as_str().starts_with("//");

        match (a, b, c) {
            (false, false, true) => {
                let after_len = self.len() - self.path_range().len() + new.len() + 2;

                if after_len > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = (after_len as u32).wrapping_sub(self.len() as u32);

                self.serialization.replace_range(self.path_range(), new.as_str());
                self.serialization.insert_str(self.path_start as usize, "/.");

                self.path_start += 2;
                if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
            },
            (false, true, false) => {
                let after_len = self.len() - self.path_range().len() + new.len() - 2;

                if after_len > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = (after_len as u32).wrapping_sub(self.len() as u32);

                self.serialization.replace_range(self.path_start as usize - 2 .. self.path_after(), new.as_str());

                self.path_start -= 2;
                if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
            },
            _ => {
                let after_len = self.len() - self.path_range().len() + new.len();

                if after_len > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = (after_len as u32).wrapping_sub(self.len() as u32);

                self.serialization.replace_range(self.path_range(), new.as_str());

                if let Some(x) = self.query_mark    {self.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.fragment_mark {self.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
            },
        }

        Ok(())
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
