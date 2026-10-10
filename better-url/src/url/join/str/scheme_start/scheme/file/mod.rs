//! [`BetterUrl::join_scheme_file`].

use crate::prelude::*;

mod abs_path;
mod rel_path;

impl BetterUrl {
    /// Join with a file scheme.
    pub(super) fn join_scheme_file(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [b'/' | b'\\', b'/' | b'\\', ..] => self.join_authority(rest),
            [b'/' | b'\\',               ..] => self.join_scheme_file_abs_path (rest),
            _                                => self.join_scheme_file_rel_path (rest),
        }
    }
}
