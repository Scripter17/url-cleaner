//! [`MyUrl::join_scheme_ns_opaque_path`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_ns_opaque_path(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = MyUrl::after_scheme(scheme, rest)?;
        Ok(())
    }
}
