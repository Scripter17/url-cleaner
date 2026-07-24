//! Value stuff.

use crate::prelude::*;

impl<'a> SpecialQuerySegment<'a> {
    /// If it has a value.
    pub fn has_value(&self) -> bool {
        self.vs.is_some()
    }

    /// The [`Range::start`] of the value.
    fn value_start(&self) -> Option<usize> {
        self.vs.map(NonZero::get)
    }

    /// The [`Range::end`] of the value.
    fn value_after(&self) -> Option<usize> {
        self.vs.map(|_| self.len())
    }

    /// The [`Range`] of the value.
    fn value_range(&self) -> Option<Range<usize>> {
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
        Some(lossy_decode_query_part(self.raw_value()?).1)
    }

    /// Consume and keep only the value.
    pub fn into_value(self) -> Option<Cow<'a, str>> {
        Some(lossy_decode_query_part(self.into_raw_value()?).1)
    }

    /// Set the value.
    pub fn set_value(&mut self, value: Option<&str>) {
        match value {
            Some(value) => {
                let (_, value) = encode_query_part(value);

                match self.value_range() {
                    Some(range) => self.raw.replace_range(range, &value),
                    None => {
                        self.vs = NonZero::new(self.len() + 1);
                        self.raw.extend(["=", &value]);
                    }
                }
            },
            None => if let Some(vs) = self.vs {
                self.raw.retain_range(..vs.get() - 1);
            }
        }
    }
}
