//! Name stuff.

use crate::prelude::*;

impl<'a> FragmentQuerySegment<'a> {
    /// The raw name.
    pub fn raw_name(&self) -> &str {
        match self.value_start {
            Some(i) => unsafe {self.as_str().get_unchecked(.. i.get() - 1)},
            None    => self.as_str()
        }
    }

    /// Consume and keep only the raw name.
    pub fn into_raw_name(self) -> Cow<'a, str> {
        let mut ret = self.raw;
        if let Some(i) = self.value_start {
            unsafe {
                ret.truncate_unchecked(i.get() - 1);
            }
        }
        ret
    }

    /// The decoded name.
    pub fn name(&self) -> Cow<'_, str> {
        let (_, name) = lossy_decode_query_part(self.raw_name());

        name
    }

    /// Consume and keep only the name.
    pub fn into_name(self) -> Cow<'a, str> {
        let (_, name) = lossy_decode_query_part(self.into_raw_name());

        name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        let (_, new) = encode_query_part(name);

        match self.value_start {
            Some(i) => {
                self.raw.replace_range(.. i.get() - 1, &new);
                self.value_start = NonZero::new(new.len() + 1);
            },
            None => self.raw.replace_range(.., &new),
        }
    }
}
