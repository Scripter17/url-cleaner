//! [`BetterUrl::join_scheme_ns`].

use crate::prelude::*;

impl BetterUrl {
    /// Join with a scheme.
    pub(super) fn join_scheme_ns(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
