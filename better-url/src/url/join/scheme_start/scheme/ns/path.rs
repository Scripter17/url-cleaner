//! [`BetterUrl::join_scheme_ns_path`].

use crate::prelude::*;

impl BetterUrl {
    /// Found a path.
    pub(super) fn join_scheme_ns_path(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
