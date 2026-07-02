use crate::prelude::*;

impl MyUrl {
    pub(crate) fn fragment_start(&self) -> Option<usize> {
        Some(self.fragment_start?.get() as usize + 1)
    }

    pub(crate) fn fragment_after(&self) -> Option<usize> {
        match self.fragment_start.is_some() {
            true  => Some(self.len()),
            false => None,
        }
    }

    pub(crate) fn fragment_range(&self) -> Option<Range<usize>> {
        Some(self.fragment_start()? .. self.fragment_after()?)
    }

    /// The fragment as a [`str`].
    pub fn fragment(&self) -> Option<&str> {
        Some(&self.serialization[self.fragment_range()?])
    }

    pub fn set_fragment<'a, T: Into<MaybeFragment<'a>>>(&mut self, value: T) -> Result<bool, SetFragmentError> {
        let old = self.fragment();
        let new = value.into();

        Ok(match (old, new.as_str()) {
            (None     , None     )               => false,
            (Some(old), Some(new)) if old == new => false,

            (None     , Some(new)) if self.len() + new.len() + 1         > u32::MAX as usize => Err(TooLong)?,
            (Some(old), Some(new)) if self.len() - old.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            (None     , Some(new)) => {
                self.fragment_start = NonZero::new(self.serialization.len() as u32);
                self.serialization.extend(["#", new]);
                true
            },
            (Some(old), None     ) => {
                self.serialization.truncate((old as *const str).addr() - (self.as_str() as *const str).addr() - 1);
                self.fragment_start = None;
                true
            },
            (Some(old), Some(new)) => {
                self.serialization.truncate(old.as_ptr().addr() - self.as_str().as_ptr().addr());
                self.serialization.push_str(new);
                true
            },
        })
    }
}
