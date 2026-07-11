//! [`BetterUrl::join_scheme_snf_relative_slash`].

use crate::prelude::*;

mod authority;
mod path;

impl BetterUrl {
    /// Found a leading slash.
    pub(super) fn join_scheme_snf_relative_slash(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [_, b'/' | b'\\', ..] => self.join_scheme_snf_relative_slash_authority(scheme, rest),
            _                     => self.join_scheme_snf_relative_slash_path     (        rest),
        }
    }
}
