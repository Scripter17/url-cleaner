//! [`MyUrl::join_scheme_file_slash`].

use crate::prelude::*;

mod authority;
mod path;

impl MyUrl {
    pub(super) fn join_scheme_file_slash(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match self.details.scheme.is_file() && !matches!(rest.as_bytes(), [b'/' | b'\\', b'/' | b'\\', ..]) {
            true  => self.join_scheme_file_slash_path     (        rest),
            false => self.join_scheme_file_slash_authority(scheme, rest),
        }
    }
}
