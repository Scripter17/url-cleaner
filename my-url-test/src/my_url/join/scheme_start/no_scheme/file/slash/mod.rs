//! [`MyUrl::join_no_scheme_file_slash`].

use crate::prelude::*;

mod host;
mod path;

impl MyUrl {
    pub(super) fn join_no_scheme_file_slash(&mut self, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [b'/' | b'\\', b'/' | b'\\', ..] => self.join_no_scheme_file_slash_host(rest),
            _                                => self.join_no_scheme_file_slash_path(rest),
        }
    }
}
