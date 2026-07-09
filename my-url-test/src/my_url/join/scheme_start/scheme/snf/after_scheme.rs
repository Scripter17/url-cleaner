//! [`MyUrl::join_scheme_snf_after_scheme`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_snf_after_scheme(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = MyUrl::after_scheme(scheme, rest)?;
        Ok(())
    }
}
