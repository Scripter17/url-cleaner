//! [`BetterUrl::join_scheme_start`].

use crate::prelude::*;

mod scheme;
mod no_scheme;

impl BetterUrl {
    /// Find the scheme.
    pub(super) fn join_scheme_start(&mut self, value: &str) -> Result<(), InvalidJoin> {
        match value.as_bytes() {
            [b'a'..=b'z' | b'A'..=b'Z', ..] if let Some(i) = value.bytes().position(|b| b == b':') && let Ok(scheme) = Scheme::new(unsafe {value.get_unchecked(..i)}) => {
                let rest = unsafe {value.get_unchecked(i+1..)};
                self.join_scheme(scheme, rest)
            },
            _ => self.join_no_scheme(value)
        }
    }
}
