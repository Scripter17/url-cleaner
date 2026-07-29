//! Value stuff.

use crate::prelude::*;

impl<'a> NonSpecialQuerySegment<'a> {
    /// If it has a value.
    pub fn has_value(&self) -> bool {
        self.value_start.is_some()
    }

    /// The raw value.
    pub fn raw_value(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.value_start?.get() ..)})
    }

    /// Consume and keep only the raw value.
    pub fn into_raw_value(self) -> Option<Cow<'a, str>> {
        let mut ret = self.raw;
        unsafe {
            ret.retain_range_unchecked(self.value_start?.get() ..);
        }
        Some(ret)
    }

    /// The decoded value.
    pub fn value(&self) -> Option<Cow<'_, str>> {
        let (_, value) = lossy_decode_query_part(self.raw_value()?);

        Some(value)
    }

    /// Consume and keep only the value.
    pub fn into_value(self) -> Option<Cow<'a, str>> {
        let (_, value) = lossy_decode_query_part(self.into_raw_value()?);

        Some(value)
    }

    /// Set the value.
    pub fn set_value(&mut self, value: Option<&str>) {
        match value {
            Some(value) => {
                let (_, value) = encode_query_part(value);

                match self.value_start {
                    Some(i) => self.raw.replace_range(i.get() .., &value),
                    None => {
                        self.value_start = NonZero::new(self.len() + 1);
                        self.raw.extend(["=", &value]);
                    }
                }
            },
            None => if let Some(i) = self.value_start {
                unsafe {
                    self.raw.truncate_unchecked(i.get() - 1);
                }
            }
        }
    }
}
