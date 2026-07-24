//! [`BetterUrl::join_authority`].

use crate::prelude::*;

impl BetterUrl {
    /// Join with an authority.
    pub(super) fn join_authority(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(self.scheme(), rest)?;

        Ok(())
    }
}
