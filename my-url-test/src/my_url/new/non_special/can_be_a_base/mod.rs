//! Can-be-a-base.

use crate::prelude::*;

mod host;
mod no_host;

impl MyUrl {
    /// Make a new non-special [`Self`] that can be a base.
    pub(super) fn new_can_be_a_base(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        match rest.as_bytes() {
            [b'/', b'/', ..] => Self::new_can_be_a_base_host   (scheme, &rest[2..]),
            _                => Self::new_can_be_a_base_no_host(scheme,  rest     ),
        }
    }
}
