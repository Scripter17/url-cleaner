//! [`BetterUrl::join_scheme_file_slash_authority`].

use crate::prelude::*;

impl BetterUrl {
    /// Found a leading authority.
    pub(super) fn join_scheme_file_slash_authority(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
