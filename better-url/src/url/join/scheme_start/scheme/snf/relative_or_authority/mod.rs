//! [`BetterUrl::join_scheme_snf_relative_or_authority`].

use crate::prelude::*;

mod relative;
mod authority;

impl BetterUrl {
    /// Found a leading relative or authority.
    pub(super) fn join_scheme_snf_relative_or_authority(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match rest.starts_with("//") {
            true  => self.join_scheme_snf_relative_or_authority_authority(scheme, rest),
            false => self.join_scheme_snf_relative                       (scheme, rest),
        }
    }
}
