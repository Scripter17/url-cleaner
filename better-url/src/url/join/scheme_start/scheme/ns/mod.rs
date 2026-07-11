//! [`BetterUrl::join_scheme_ns`].

use crate::prelude::*;

mod opaque_path;
mod path;
mod authority;

impl BetterUrl {
    /// `scheme` is [`SchemeType::NonSpecial`].
    pub(super) fn join_scheme_ns(&mut self, scheme: Scheme<'_>, rest: &str) -> Result<(), InvalidJoin> {
        match rest.as_bytes() {
            [b'/', b'/', ..]  => self.join_scheme_ns_authority  (scheme, rest),
            [b'/',       ..]  => self.join_scheme_ns_path       (scheme, rest),
            _                 => self.join_scheme_ns_opaque_path(scheme, rest),
        }
    }
}
