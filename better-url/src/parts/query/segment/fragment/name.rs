//! Name stuff.

use crate::prelude::*;

impl<'a> FragmentQuerySegment<'a> {
    /// The [`FragmentQueryName`].
    pub fn name(&self) -> FragmentQueryName<'_> {
        let ret = match self.value_start {
            Some(i) => unsafe {self.as_str().get_unchecked(.. i.get() - 1)},
            None    => self.as_str()
        };

        unsafe {
            FragmentQueryName::new_unchecked(ret)
        }
    }

    /// Turn into the [`FragmentQueryName`].
    pub fn into_name(self) -> FragmentQueryName<'a> {
        let mut ret = self.raw;

        if let Some(i) = self.value_start {
            unsafe {
                ret.truncate_unchecked(i.get() - 1);
            }
        }

        unsafe {
            FragmentQueryName::new_unchecked(ret)
        }
    }

    /// Set the name.
    pub fn set_name<'b, T: Into<FragmentQueryName<'b>>>(&mut self, name: T) {
        let new = name.into();

        match self.value_start {
            Some(i) => {
                self.raw.replace_range(.. i.get() - 1, new.as_str());
                self.value_start = NonZero::new(new.len() + 1);
            },
            None => self.raw.replace_range(.., new.as_str()),
        }
    }
}
