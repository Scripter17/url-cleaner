//! [`MyUrl::join_scheme_ns_path_or_authority_path`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_ns_path_or_authority_path(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = Self::after_scheme(scheme, rest)?;
        Ok(())
    }
}
