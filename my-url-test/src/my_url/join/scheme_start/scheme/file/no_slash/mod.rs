//! [`MyUrl::join_scheme_file_no_slash`].

use crate::prelude::*;

mod file;
mod not_file;

impl MyUrl {
    pub(super) fn join_scheme_file_no_slash(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match self.details.scheme.is_file() {
            true  => self.join_scheme_file_no_slash_from_file    (        rest),
            false => self.join_scheme_file_no_slash_from_not_file(scheme, rest),
        }
    }
}
