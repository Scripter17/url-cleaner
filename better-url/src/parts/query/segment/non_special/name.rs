//! Name stuff.

use crate::prelude::*;

impl<'a> NonSpecialQuerySegment<'a> {
    /// The [`Range::start`] of the name.
    fn name_start(&self) -> usize {
        0
    }

    /// The [`Range::end`] of the name.
    fn name_after(&self) -> usize {
        match self.vs {
            None    => self.len(),
            Some(x) => x.get() - 1
        }
    }

    /// The [`Range`] of the name.
    fn name_range(&self) -> Range<usize> {
        self.name_start() .. self.name_after()
    }

    /// The raw name.
    pub fn raw_name(&self) -> &str {
        &self.as_str()[self.name_range()]
    }

    /// Consume and keep only the raw name.
    pub fn into_raw_name(self) -> Cow<'a, str> {
        let range = self.name_range();
        let mut ret = self.raw;
        ret.retain_range(range);
        ret
    }

    /// The decoded name.
    pub fn name(&self) -> Cow<'_, str> {
        lossy_decode_query_part(self.raw_name()).1
    }

    /// Consume and keep only the name.
    pub fn into_name(self) -> Cow<'a, str> {
        lossy_decode_query_part(self.into_raw_name()).1
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        let name = encode_query_part(name).1;
        self.raw.replace_range(self.name_range(), &name);
        if self.has_value() {
            self.vs = NonZero::new(name.len() + 1);
        }
    }
}
