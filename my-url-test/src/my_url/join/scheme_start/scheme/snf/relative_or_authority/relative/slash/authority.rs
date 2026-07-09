//! [`MyUrl::join_scheme_snf_relative_slash_authority`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_snf_relative_slash_authority(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = MyUrl::after_scheme(scheme, rest)?;
        Ok(())
    }
}
