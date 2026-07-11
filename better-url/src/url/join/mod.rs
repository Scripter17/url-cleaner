//! [`BetterUrl::join`].

use crate::prelude::*;

mod scheme_start;

impl BetterUrl {
    /// Join in-place.
    /// # Errors
    /// If `value` isn't a valid join, returns the error [`InvalidJoin`].
    pub fn join(&mut self, value: &str) -> Result<(), InvalidJoin> {
        let start = value.bytes(). position(|b| b > 0x20 && b != 0x7F).unwrap_or(0);
        let end   = value.bytes().rposition(|b| b > 0x20 && b != 0x7F).map_or(0, |x| x + 1);

        let mut value = Cow::Borrowed(&value[start..end]);

        if value.bytes().any(|b| b == b'\t' || b == b'\n' || b == b'\r') {
            value.to_mut().retain(|c| c != '\t' && c != '\n' && c != '\r');
        }

        self.join_scheme_start(&value)
    }
}
