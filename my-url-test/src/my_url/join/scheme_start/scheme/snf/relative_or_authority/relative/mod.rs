//! [`MyUrl::join_scheme_snf_relative`].

use crate::prelude::*;

mod slash;
mod not_slash;

impl MyUrl {
    pub(super) fn join_scheme_snf_relative(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [b'/' | b'\\', ..] => self.join_scheme_snf_relative_slash    (scheme, rest),
            _                  => self.join_scheme_snf_relative_not_slash(        rest),
        }
    }
}
