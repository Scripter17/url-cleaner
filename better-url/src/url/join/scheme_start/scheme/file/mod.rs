//! [`BetterUrl::join_scheme_file`].

use crate::prelude::*;

mod slash;
mod no_slash;

impl BetterUrl {
    /// The scheme is `file`.
    pub(super) fn join_scheme_file(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [b'/' | b'\\', ..] => self.join_scheme_file_slash   (scheme, rest),
            _                  => self.join_scheme_file_no_slash(scheme, rest),
        }
    }
}
