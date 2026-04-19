//! Value stuff.

use crate::prelude::*;

impl<'a> SpecialQuerySegment<'a> {
    /// If it has a value.
    pub fn has_value(&self) -> bool {
        self.vs != 0
    }
    
    /// The [`Range::start`] of the value.
    pub fn value_start(&self) -> Option<usize> {
        match self.vs {
            0 => None,
            x => Some(x)
        }
    }

    /// The [`Range::end`] of the value.
    pub fn value_after(&self) -> Option<usize> {
        match self.vs {
            0 => None,
            _ => Some(self.len())
        }
    }

    /// The [`Range`] of the value.
    pub fn value_range(&self) -> Option<Range<usize>> {
        Some(self.value_start()? .. self.value_after()?)
    }

    /// The raw value.
    pub fn raw_value(&self) -> Option<&str> {
        self.value_range().map(|r| &self.as_str()[r])
    }

    /// Consume and keep only the raw value.
    pub fn into_raw_value(self) -> Option<Cow<'a, str>> {
        let range = self.value_range()?;
        let mut ret = self.raw;
        ret.retain_range(range);
        Some(ret)
    }

    /// The decoded value.
    pub fn value(&self) -> Option<Cow<'_, str>> {
        Some(PartTranscoder::QueryPart.decode_lossy(self.raw_value()?.into()))
    }

    /// Consume and keep only the value.
    pub fn into_value(self) -> Option<Cow<'a, str>> {
        Some(PartTranscoder::QueryPart.decode_lossy(self.into_raw_value()?))
    }

    /// Set the value.
    pub fn set_value(&mut self, value: &str) {
        let value = PartTranscoder::QueryPart.encode(value.into());
        match self.value_range() {
            Some(range) => self.raw.replace_range(range, &value),
            None => {
                self.vs = self.len() + 1;
                self.raw.to_mut().extend(["=", &value]);
            }
        }
    }
}
