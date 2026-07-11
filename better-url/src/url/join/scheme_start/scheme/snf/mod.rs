//! [`BetterUrl::join_scheme_snf`].

use crate::prelude::*;

mod relative_or_authority;
mod after_scheme;

impl BetterUrl {
    /// `scheme` is [`SchemeType::SpecialNotFile`].
    pub(super) fn join_scheme_snf(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match self.details.scheme == scheme.details() {
            true  => self.join_scheme_snf_relative_or_authority(scheme, rest),
            false => self.join_scheme_snf_after_scheme         (scheme, rest),
        }
    }
}
