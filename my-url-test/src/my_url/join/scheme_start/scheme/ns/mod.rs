//! [`MyUrl::join_scheme_ns`].

use crate::prelude::*;

mod opaque_path;
mod path_or_authority;

impl MyUrl {
    pub(super) fn join_scheme_ns(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match rest.starts_with('/') {
            true  => self.join_scheme_ns_path_or_authority(scheme, rest),
            false => self.join_scheme_ns_opaque_path      (scheme, rest),
        }
    }
}
