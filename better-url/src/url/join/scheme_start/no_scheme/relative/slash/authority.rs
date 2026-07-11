//! [`BetterUrl::join_no_scheme_relative_slash_authority`].

use crate::prelude::*;

impl BetterUrl {
    /// Found a leading authority.
    pub(super) fn join_no_scheme_relative_slash_authority(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(self.scheme(), rest)?;
        Ok(())
    }
}
