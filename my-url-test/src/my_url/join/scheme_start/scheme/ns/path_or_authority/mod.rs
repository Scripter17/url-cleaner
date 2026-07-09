//! [`MyUrl::join_scheme_ns_path_or_authority`].

use crate::prelude::*;

mod path;
mod authority;

impl MyUrl {
    pub(super) fn join_scheme_ns_path_or_authority(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match rest.starts_with("//") {
            true  => self.join_scheme_ns_path_or_authority_authority(scheme, rest),
            false => self.join_scheme_ns_path_or_authority_path     (scheme, rest),
        }
    }
}
