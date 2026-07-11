//! [`BetterUrl::join_scheme_ns_opaque_path`].

use crate::prelude::*;

impl BetterUrl {
    /// Found an opaque path.
    pub(super) fn join_scheme_ns_opaque_path(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
