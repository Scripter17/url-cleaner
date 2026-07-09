//! [`MyUrl::join_scheme_ns_path_or_authority_authority`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_ns_path_or_authority_authority(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = MyUrl::after_scheme(scheme, rest)?;
        Ok(())
    }
}
