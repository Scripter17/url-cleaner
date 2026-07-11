//! [`BetterUrl::join_scheme_snf_after_scheme`].

use crate::prelude::*;

impl BetterUrl {
    /// `self` has the same scheme as `scheme`.
    pub(super) fn join_scheme_snf_after_scheme(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
