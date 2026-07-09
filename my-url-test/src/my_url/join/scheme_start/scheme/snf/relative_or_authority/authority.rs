//! [`MyUrl::join_scheme_snf_relative_or_authority_authority`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_snf_relative_or_authority_authority(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
