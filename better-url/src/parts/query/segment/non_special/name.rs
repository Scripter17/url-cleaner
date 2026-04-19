//! Name stuff.

use crate::prelude::*;

impl<'a> NonSpecialQuerySegment<'a> {
    /// The [`Range::start`] of the name.
    pub fn name_start(&self) -> usize {
        0
    }

    /// The [`Range::end`] of the name.
    pub fn name_after(&self) -> usize {
        match self.vs {
            0 => self.len(),
            x => x - 1
        }
    }

    /// The [`Range`] of the name.
    pub fn name_range(&self) -> Range<usize> {
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
        PartTranscoder::QueryPart.decode_lossy(self.raw_name().into())
    }

    /// Consume and keep only the name.
    pub fn into_name(self) -> Cow<'a, str> {
        PartTranscoder::QueryPart.decode_lossy(self.into_raw_name())
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        let name = PartTranscoder::QueryPart.encode(name.into());
        self.raw.replace_range(self.name_range(), &name);
        if self.has_value() {
            self.vs = name.len() + 1;
        }
    }
}
