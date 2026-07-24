//! Non-special URLs.

use crate::prelude::*;

mod host;
mod path;
mod opaque_path;

impl BetterUrl {
    /// [`SchemeType::NonSpecial`].
    pub(super) fn new_non_special(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match rest.as_bytes() {
            [b'/', b'/', ..]  => Self::new_ns_host       (scheme, &rest[2..]),
            [b'/',       ..]  => Self::new_ns_path       (scheme, rest),
            _                 => Self::new_ns_opaque_path(scheme, rest),
        }
    }
}
