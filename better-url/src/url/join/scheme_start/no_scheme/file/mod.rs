//! [`BetterUrl::join_no_scheme_file`].

use crate::prelude::*;

mod slash;
mod no_slash;

impl BetterUrl {
    /// `self` is a file URL.
    pub(super) fn join_no_scheme_file(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [b'/' | b'\\', ..] => self.join_no_scheme_file_slash   (rest),
            _                  => self.join_no_scheme_file_no_slash(rest)
        }
    }
}
