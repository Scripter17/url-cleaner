//! Non-special URLs.

use crate::prelude::*;

mod host;
mod path;
mod opaque_path;

impl BetterUrl {
    /// [`SchemeType::NonSpecial`].
    pub(super) fn new_non_special_str(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match rest.as_bytes() {
            [b'/', b'/', ..]  => Self::new_ns_host_str       (scheme, &rest[2..]),
            [b'/',       ..]  => Self::new_ns_path_str       (scheme, rest),
            _                 => Self::new_ns_opaque_path_str(scheme, rest),
        }
    }
}
