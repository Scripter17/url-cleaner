//! [`BetterUrl::join_no_scheme_relative`].

use crate::prelude::*;

mod slash;
mod not_slash;

impl BetterUrl {
    /// `self` is relative.
    pub(super) fn join_no_scheme_relative(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match (rest.as_bytes(), self.details.scheme.is_special()) {
            ([b'/' | b'\\', ..], true) | ([b'/', ..], false) => self.join_no_scheme_relative_slash    (rest),
            _                                                => self.join_no_scheme_relative_not_slash(rest),
        }
    }
}
