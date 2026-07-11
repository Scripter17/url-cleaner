//! [`BetterUrl::join_no_scheme_file_slash_host`].

use crate::prelude::*;

impl BetterUrl {
    /// Found a leading host.
    pub(super) fn join_no_scheme_file_slash_host(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(self.scheme(), rest)?;
        Ok(())
    }
}
