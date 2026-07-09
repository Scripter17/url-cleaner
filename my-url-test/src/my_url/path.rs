//! [`Path`] and co..

use crate::prelude::*;

impl MyUrl {
    /// The [`Range::start`] of the path.
    fn path_start(&self) -> usize {
        self.path_start as usize
    }

    /// The [`Range::end`] of the path.
    fn path_after(&self) -> usize {
        self.query_mark.or(self.fragment_mark).map_or(self.len(), |x| x.get() as usize)
    }

    /// The [`Range`] of the path.
    fn path_range(&self) -> Range<usize> {
        self.path_start() .. self.path_after()
    }

    /// The path as a [`str`].
    pub fn path(&self) -> &str {
        &self.serialization[self.path_range()]
    }

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
}
