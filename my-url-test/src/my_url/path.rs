use crate::prelude::*;

impl MyUrl {
    fn path_start(&self) -> usize {
        self.path_start as usize
    }

    fn path_after(&self) -> usize {
        self.query_start.or(self.fragment_start).map(NonZero::get).unwrap_or(self.len() as u32) as usize
    }

    fn path_range(&self) -> Range<usize> {
        self.path_start() .. self.path_after()
    }

    /// The path as a [`str`].
    pub fn path(&self) -> &str {
        &self.serialization[self.path_range()]
    }

    pub fn new_path<'a, T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>> + Into<OpaquePath<'a>>>(&self, path: T) -> Path<'a> {
        match self.cannot_be_a_base() {
            true  => Path::new_opaque(path),
            false => match self.details.scheme.r#type() {
                SchemeType::File           => Path::new_file            (path),
                SchemeType::SpecialNotFile => Path::new_special_not_file(path),
                SchemeType::NonSpecial     => Path::new_non_special     (path),
            }
        }
    }

    pub fn set_path<'a, T: Into<FilePath<'a>> + Into<SpecialNotFilePath<'a>> + Into<NonSpecialPath<'a>> + Into<OpaquePath<'a>>>(&mut self, path: T) -> Result<bool, SetPathError> {
        let old = self.path();
        let new = self.new_path(path);

        if old == new {
            return Ok(false);
        }

        let old_len = old.len();
        let new_len = new.len();

        let after_len = self.len() - old_len + new_len;

        if after_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        self.serialization.replace_range(self.path_range(), new.as_str());

        if let Some(x) = self.query_start    {self.query_start    = NonZero::new(x.get() - old_len as u32 + new_len as u32);}
        if let Some(x) = self.fragment_start {self.fragment_start = NonZero::new(x.get() - old_len as u32 + new_len as u32);}

        Ok(true)
    }
}
