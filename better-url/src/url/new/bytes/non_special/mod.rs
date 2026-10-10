//! Non-special URLs.

use crate::prelude::*;

mod host;
mod path;
mod opaque_path;

impl BetterUrl {
    /// [`SchemeType::NonSpecial`].
    pub(super) fn new_non_special_bytes(scheme: Scheme<'_>, rest: &[u8]) -> Result<Self, InvalidUrl> {
        match rest {
            [b'/', b'/', ..]  => Self::new_ns_host_bytes       (scheme, &rest[2..]),
            [b'/',       ..]  => Self::new_ns_path_bytes       (scheme, rest),
            _                 => Self::new_ns_opaque_path_bytes(scheme, rest),
        }
    }
}
