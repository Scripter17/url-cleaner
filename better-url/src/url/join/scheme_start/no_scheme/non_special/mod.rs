//! [`BetterUrl::join_no_scheme_ns`].

use crate::prelude::*;

mod abs_path;
mod rel_path;

impl BetterUrl {
    /// Join without a scheme.
    pub(super) fn join_no_scheme_ns(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [b'/', b'/', ..] => self.join_authority            (rest),
            [b'/'      , ..] => self.join_no_scheme_ns_abs_path(rest),
            _                => self.join_no_scheme_ns_rel_path(rest),
        }
    }
}
