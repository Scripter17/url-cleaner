//! Value stuff.

use crate::prelude::*;

impl<'a> SpecialQuerySegment<'a> {
    /// If it has a value.
    pub fn has_value(&self) -> bool {
        self.value_start.is_some()
    }

    /// The [`MaybeSpecialQueryValue`].
    pub fn value(&self) -> MaybeSpecialQueryValue<'_> {
        unsafe {
            MaybeSpecialQueryValue::new_unchecked(self.value_start.map(|x| self.as_str().get_unchecked(x.get() ..)))
        }
    }

    /// Turn into the [`MaybeSpecialQueryValue`].
    pub fn into_value(self) -> MaybeSpecialQueryValue<'a> {
        let mut ret = self.raw;

        match self.value_start {
            Some(x) => unsafe {
                ret.retain_range_unchecked(x.get() ..);

                MaybeSpecialQueryValue::new_unchecked(Some(ret))
            },
            None => MaybeSpecialQueryValue(None)
        }
    }

    /// Set the value.
    pub fn set_value<'b, T: Into<MaybeSpecialQueryValue<'b>>>(&mut self, value: T) {
        match value.into().as_str() {
            Some(value) => {
                match self.value_start {
                    Some(i) => self.raw.replace_range(i.get() .., value),
                    None => {
                        self.value_start = NonZero::new(self.len() + 1);
                        self.raw.extend(["=", value]);
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
