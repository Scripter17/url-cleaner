//! [`MyUrl::join_scheme_file_no_slash_from_not_file`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_scheme_file_no_slash_from_not_file(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        *self = MyUrl::after_scheme(scheme, rest)?;
        Ok(())
    }
}
