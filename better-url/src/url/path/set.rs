//! Setters.

use crate::prelude::*;

impl BetterUrl {
    /// Set the path.
    /// # Errors
    /// If the URL would become too long, returns the error [`TooLong`].
    pub fn set_path<'a, T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>>>(&mut self, path: T) -> Result<(), SetPathError> {
        if self.cannot_be_a_base() {
            return Ok(());
        }

        let mut new = match self.details.scheme.r#type() {
            SchemeType::File           => Path::new_file            (path),
            SchemeType::SpecialNotFile => Path::new_special_not_file(path),
            SchemeType::NonSpecial     => Path::new_non_special     (path),
        };

        let a = self.has_host();
        let b = unsafe {self.as_str().get_unchecked(self.details.scheme_mark as usize .. self.details.path_start as usize)};

        if !a && new.is_empty() {
            new = unsafe {NonSpecialPath::new_unchecked("/")}.into();
        }

        let old_range = self.path_range();

        match a {
            false if b != ":/." && new.as_str().starts_with("//") => {
                let after_len = self.len() - old_range.len() + new.len() + 2;

                if after_len > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = (after_len as u32).wrapping_sub(self.len() as u32);

                unsafe {
                    self.serialization.as_mut_vec().replace_range_with_unchecked(old_range.clone(), &[b"/.", new.as_str().as_bytes()]);
                }

                self.details.path_start += 2;
                if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
            },
            false if b == ":/." && !new.as_str().starts_with("//") => {
                let after_len = self.len() - old_range.len() + new.len() - 2;

                if after_len > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = (after_len as u32).wrapping_sub(self.len() as u32);

                unsafe {
                    self.serialization.as_mut_vec().replace_range_unchecked(old_range.start - 2 .. old_range.end, new.as_str().as_bytes());
                }

                self.details.path_start -= 2;
                if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
            },
            _ => {
                let after_len = self.len() - old_range.len() + new.len();

                if after_len > u32::MAX as usize {
                    Err(TooLong)?;
                }

                let diff = (after_len as u32).wrapping_sub(self.len() as u32);

                unsafe {
                    self.serialization.as_mut_vec().replace_range_unchecked(old_range, new.as_str().as_bytes());
                }

                if let Some(x) = self.details.query_mark    {self.details.query_mark    = NonZero::new(x.get().wrapping_add(diff));}
                if let Some(x) = self.details.fragment_mark {self.details.fragment_mark = NonZero::new(x.get().wrapping_add(diff));}
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
    pub fn set_path_segment<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetPathError> {
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
    pub fn set_path_range<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.set_range(range, value)? {
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
    pub fn insert_path_segment<'a, T: Into<SpecialNotFilePathSegments<'a>> + Into<FilePathSegments<'a>> + Into<NonSpecialPathSegments<'a>>>(&mut self, index: isize, value: T) -> Result<bool, SetPathError> {
        let mut path = self.segmented_path().ok_or(PathIsOpaque)?;

        if path.insert(index, value)? {
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
