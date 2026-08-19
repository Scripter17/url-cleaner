//! Removers.

use crate::prelude::*;

impl NonSpecialPath<'_> {
    /// Remove the last segment if it exists.
    pub fn pop(&mut self) -> bool {
        match self.0.memrchr(b'/') {
            Some(i) => {
                unsafe {
                    self.0.truncate_unchecked(i);
                }

                true
            },
            None => false
        }
    }

    /// Remove the last segment if it exists and is empty.
    pub fn pop_if_empty(&mut self) -> bool {
        match self.as_str().as_bytes() {
            [x @ .., b'/'] => {
                unsafe {
                    self.0.truncate_unchecked(x.len())
                };

                true
            },
            _ => false
        }
    }

    /// Remove the `index`th segment if it exists.
    pub fn remove(&mut self, index: isize) -> bool {
        match self.get_str(index) {
            Some(temp) => {
                let mut range = self.as_str().my_substr_range(temp);
                range.start -= 1;

                self.0.replace_range(range, "");

                true
            },
            None => false
        }
    }
}
