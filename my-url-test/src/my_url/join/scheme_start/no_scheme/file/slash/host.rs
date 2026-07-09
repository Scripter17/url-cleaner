//! [`MyUrl::join_no_scheme_file_slash_host`].

use crate::prelude::*;

impl MyUrl {
    pub(super) fn join_no_scheme_file_slash_host(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        *self = MyUrl::after_scheme("file".parse()?, rest)?;
        Ok(())
    }
}
