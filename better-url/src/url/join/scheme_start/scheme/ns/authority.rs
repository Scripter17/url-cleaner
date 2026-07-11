//! [`BetterUrl::join_scheme_ns_authority`].

use crate::prelude::*;

impl BetterUrl {
    /// Found an authority.
    pub(super) fn join_scheme_ns_authority(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
