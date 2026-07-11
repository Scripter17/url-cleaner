//! [`BetterUrl::join_no_scheme_relative_slash`].

use crate::prelude::*;

mod authority;
mod path;

impl BetterUrl {
    /// Found a leading slash.
    pub(super) fn join_no_scheme_relative_slash(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match (rest.as_bytes(), self.details.scheme.is_special()) {
            ([_, b'/' | b'\\', ..], true ) | ([_, b'/', ..], false) => self.join_no_scheme_relative_slash_authority(rest),
            _                                                       => self.join_no_scheme_relative_slash_path(rest),
        }
    }
}
